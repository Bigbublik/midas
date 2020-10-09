use ::async_trait::async_trait;
use ::futures::sink::SinkExt;
use ::nats::asynk::{Connection as Broker, Subscription as NatsSub};
use ::rand::random;
use ::serde_json::to_vec as to_json;
use ::slog::Logger;
use ::tokio::net::TcpStream;
use ::tokio_tungstenite::{
  connect_async, tungstenite as wsocket, WebSocketStream,
};
use ::tonic::transport::Channel;
use ::tonic::Request;

use ::rpc::entities::Exchanges;
use ::rpc::symbol::symbol_client::SymbolClient;
use ::rpc::symbol::ListRequest;
use ::types::{ret_on_err, GenericResult, SendableErrorResult};

use super::constants::{TRADE_OBSERVER_SUB_NAME, WS_ENDPOINT};
use super::entities::{TradeSubRequest, TradeSubRequestInner};

use crate::errors::{AlreadyStarted, WebsocketError};
use crate::traits::TradeObserver as TradeObserverTrait;

pub struct TradeObserver {
  id: u32,
  broker: Broker,
  logger: Logger,
  symbols: Vec<String>,
}

impl TradeObserver {
  pub async fn new(
    broker: Broker,
    logger: Logger,
    symbol_client: &mut SymbolClient<Channel>,
  ) -> SendableErrorResult<Self> {
    let symbols = ret_on_err!(
      symbol_client
        .list(Request::new(ListRequest {
          exchange: Exchanges::Binance as i32,
        }))
        .await
    )
    .into_inner()
    .symbols
    .into_iter()
    .map(|item| item.symbol)
    .collect();

    return Ok(Self {
      id: random(),
      broker,
      logger,
      symbols,
    });
  }

  async fn init_socket(
    &self,
  ) -> SendableErrorResult<WebSocketStream<TcpStream>> {
    let (websocket, resp) = ret_on_err!(connect_async(WS_ENDPOINT).await);
    let status = resp.status();
    if !status.is_informational() {
      return Err(Box::new(WebsocketError { status }));
    }
    return Ok(websocket);
  }

  async fn init_subscription(
    &self,
    socket: &mut WebSocketStream<TcpStream>,
  ) -> SendableErrorResult<()> {
    let mut inner = TradeSubRequestInner {
      id: self.id,
      params: vec![],
    };
    for symbol in &self.symbols {
      inner
        .params
        .push(format!("{}@{}", symbol.to_lowercase(), "trade"));
    }
    let req = TradeSubRequest::Subscribe(inner);
    let payload = ret_on_err!(to_json(&req));
    let payload = ret_on_err!(String::from_utf8(payload));
    let req = wsocket::Message::Text(payload);
    let _ = socket.send(req).await;
    let _ = socket.flush().await;
    return Ok(());
  }
}

#[async_trait]
impl TradeObserverTrait for TradeObserver {
  async fn start(&self) -> SendableErrorResult<NatsSub> {
    let mut socket = self.init_socket().await?;
    self.init_subscription(&mut socket).await?;
    let sub = ret_on_err!(self.broker.subscribe(TRADE_OBSERVER_SUB_NAME).await);
    return Ok(sub);
  }
  async fn stop(&self) {}
}
