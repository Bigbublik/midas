use ::std::sync::Arc;
use ::std::time::Duration as StdDur;

use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::future::{join, join_all, BoxFuture};
use ::futures::stream::BoxStream;
use ::futures::{FutureExt, StreamExt};
use ::mongodb::bson::{doc, oid::ObjectId, to_document, DateTime};
use ::mongodb::options::{UpdateModifications, UpdateOptions};
use ::mongodb::{Collection, Database};
use ::rug::Float;
use ::serde_qs::to_string as to_qs;

use ::entities::{
  BookTicker, ExecutionSummary, Order, OrderInner, OrderOption,
};
use ::errors::{ExecutionResult, HTTPErrors, StatusFailure};
use ::keychain::{IKeyChain, KeyChain};
use ::round_robin_client::RestClient;
use ::rpc::exchanges::Exchanges;
use ::writers::DatabaseWriter;

use ::clients::binance::{APIHeader, REST_ENDPOINTS};
use ::observers::binance::TradeSubscriber;
use ::observers::traits::ITradeSubscriber as TradeSubscriberTrait;
use ::subscribe::nats::Client as Nats;

use crate::traits::Executor as ExecutorTrait;

use super::interfaces::INewOrderRequestMaker;
use super::services::NewOrderRequestMaker;

use super::entities::{
  CancelOrderRequest, OrderRequest, OrderResponse, OrderType, Side,
};

pub struct Executor {
  keychain: Arc<dyn IKeyChain + Send + Sync>,
  req_maker: Arc<dyn INewOrderRequestMaker + Send + Sync>,
  broker: Nats,
  db: Database,
  positions: Collection<OrderResponse<Float, DateTime>>,
  cli: RestClient,
}

impl Executor {
  pub async fn new(broker: &Nats, db: Database) -> ExecutionResult<Self> {
    let keychain = Arc::new(KeyChain::new(broker, db.clone()).await?);
    let req_maker = Arc::new(NewOrderRequestMaker::new());

    let positions = db.collection("binance.positions");
    let me = Self {
      keychain,
      broker: broker.clone(),
      req_maker,
      db,
      positions,
      cli: RestClient::new(
        REST_ENDPOINTS
          .into_iter()
          .filter_map(|&url| format!("{}/api/v3/order", url).parse().ok())
          .collect(),
        StdDur::from_secs(5),
        StdDur::from_secs(5),
      )?,
    };
    me.update_indices(&[
      "orderId",
      "clientOrderId",
      "settlementGid",
      "positionGroupId",
    ])
    .await;
    return Ok(me);
  }
}

impl APIHeader for Executor {}

impl DatabaseWriter for Executor {
  fn get_database(&self) -> &Database {
    return &self.db;
  }
  fn get_col_name(&self) -> &str {
    return self.positions.name();
  }
}

#[async_trait]
impl ExecutorTrait for Executor {
  async fn open(
    &mut self,
  ) -> ExecutionResult<BoxStream<'_, ExecutionResult<BookTicker>>> {
    let stream = try_stream! {
      let observer = TradeSubscriber::new(
      &self.broker,
    )
    .await?;
    let mut sub = observer.subscribe().await?;
      while let Some(book_ticker) = sub.next().await {
        yield book_ticker;
      }
    };
    return Ok(Box::pin(stream));
  }
  async fn create_order(
    &mut self,
    api_key_id: ObjectId,
    symbol: String,
    price: Option<Float>,
    budget: Float,
    order_option: Option<OrderOption>,
  ) -> ExecutionResult<ObjectId> {
    let api_key = self.keychain.get(Exchanges::Binance, api_key_id).await?;
    let pos_gid = ObjectId::new();
    let header = self.get_pub_header(&api_key.inner())?;
    let resp_defers: Vec<BoxFuture<ExecutionResult<usize>>> =
      self
        .req_maker
        .build(&api_key, symbol, budget, price, order_option)
        .await?
        .into_iter()
        .map(|qs| {
          let header = header.clone();
          let mut cli = self.cli.clone();
          return async move {
            (cli.post(Some(header.clone()), Some(qs)).await, cli)
          };
        })
        .map(|fut| {
          let pos = self.positions.clone();
          return fut
            .then(|(resp, cli)| async move {
              let resp = resp?;
              let payload: OrderResponse<String, i64> = resp.json().await?;
              let mut payload =
                OrderResponse::<Float, DateTime>::try_from(payload)?;
              payload.position_group_id = Some(pos_gid.clone());
              let _ = pos
                .update_one(
                  doc! {"orderId": payload.order_id},
                  UpdateModifications::Document(to_document(&payload)?),
                  UpdateOptions::builder().upsert(true).build(),
                )
                .await;
              return Ok(cli.get_state());
            })
            .boxed();
        })
        .collect();
    let result = join_all(resp_defers).await;
    let state = result.iter().filter_map(|res| res.as_ref().ok()).max();
    if let Some(state) = state {
      self.cli.set_state(*state);
    }
    let res_err = result.into_iter().find(|item| item.is_err());
    return match res_err {
      Some(e) => Err(e.unwrap_err()),
      None => Ok(pos_gid),
    };
  }

  async fn remove_order(
    &mut self,
    api_key_id: ObjectId,
    gid: ObjectId,
  ) -> ExecutionResult<ExecutionSummary> {
    let api_key = self.keychain.get(Exchanges::Binance, api_key_id).await?;
    let mut positions = self
      .positions
      .find(doc! {"positionGroupId": gid}, None)
      .await?
      .filter_map(|pos| async { pos.ok() })
      .boxed();
    let mut order_cancel_vec = vec![];
    let mut position_reverse_vec = vec![];
    while let Some(pos) = positions.next().await {
      // Cancel Order
      let symbol = pos.symbol.clone();
      let order_id = pos.order_id.clone();
      order_cancel_vec.push({
        let api_key = api_key.clone();
        let mut cli = self.cli.clone();
        async move {
          let req =
            CancelOrderRequest::<i64>::new(symbol).order_id(Some(order_id));
          let qs = to_qs(&req)?;
          let qs = format!(
            "{}&signature={}",
            qs,
            api_key.sign(Exchanges::Binance, &qs)
          );
          let resp = cli.delete(None, Some(qs)).await?;
          let status = resp.status();
          if !status.is_success() {
            return Err(
              StatusFailure {
                url: Some(resp.url().to_string()),
                code: status.as_u16(),
                text: resp
                  .text()
                  .await
                  .unwrap_or("Failed to get the text".to_string()),
              }
              .into(),
            );
          }
          return Ok((resp, cli.get_state()));
        }
      });
      let symbol = pos.symbol.clone();
      if let Some(fills) = &pos.fills {
        // Sell the position
        let qty_to_reverse = fills
          .into_iter()
          .map(|item| &item.qty)
          .fold(Float::with_val(32, 0.0), |acc, v| acc + v);
        let req = OrderRequest::<i64>::new(
          symbol.clone(),
          Side::Sell,
          OrderType::Market,
        )
        .quantity(Some(qty_to_reverse.to_string()));
        let qs = to_qs(&req)?;
        let qs =
          format!("{}&signature={}", qs, api_key.sign(Exchanges::Binance, &qs));
        let pos: Order = pos.clone().into();
        let pos_pur_price: OrderInner = pos.clone().sum();
        position_reverse_vec.push({
          let mut cli = self.cli.clone();
          async move {
            let resp = cli.post(None, Some(&qs)).await?;
            let status = resp.status();
            if !status.is_success() {
              return Err(
                StatusFailure {
                  url: Some(resp.url().to_string()),
                  code: status.as_u16(),
                  text: resp
                    .text()
                    .await
                    .unwrap_or("Failed to get the text".to_string()),
                }
                .into(),
              );
            }
            let rev_order_resp: OrderResponse<String, i64> = resp
              .json()
              .await
              .map_err(|e| HTTPErrors::RequestFailure(e))?;
            let rev_order_resp =
              OrderResponse::<Float, DateTime>::try_from(rev_order_resp)?;
            let rev_order_resp: Order = rev_order_resp.into();
            let rev_pos_price: OrderInner = rev_order_resp.sum();
            return Ok((pos_pur_price, rev_pos_price, cli.get_state()));
          }
        });
      };
    }
    let (order_res, position_res) =
      join(join_all(order_cancel_vec), join_all(position_reverse_vec)).await;
    let order_state = order_res
      .iter()
      .filter_map(|res| res.as_ref().ok())
      .map(|(_, state)| *state)
      .max();
    let pos_state = position_res
      .iter()
      .filter_map(|res| res.as_ref().ok())
      .map(|(_, _, state)| *state)
      .max();
    if let Some(state) = order_state {
      self.cli.set_state(state);
    }
    if let Some(state) = pos_state {
      if self.cli.get_state() >= state {
        self.cli.set_state(state);
      }
    }
    for order_res in order_res {
      if order_res.is_err() {
        return Err(order_res.err().unwrap());
      }
    }
    let mut pur_order = OrderInner::default();
    let mut sell_order = OrderInner::default();
    for position_res in position_res {
      let (pur, sell, _) = match position_res {
        Err(e) => return Err(e),
        Ok(o) => o,
      };
      pur_order += pur;
      sell_order += sell;
    }
    return Ok(ExecutionSummary::calculate_profit(&sell_order, &pur_order));
  }
}
