use ::std::collections::hash_map::HashMap;

use ::chrono::{DateTime as ChronoDateTime, Utc};
use ::futures::StreamExt;
use ::mongodb::bson::{doc, from_document, to_bson, DateTime as MongoDateTime};
use ::mongodb::Collection;
use ::nats::asynk::Connection as NatsConnection;
use ::rmp_serde::from_slice as from_msgpack;
use ::slog::{crit, Logger};
use ::tokio::select;
use ::tokio::sync::{broadcast, mpsc};
use ::tokio::task::block_in_place;

use ::rpc::historical::HistChartProg;
use ::types::{ret_on_err, SendableErrorResult};

use super::constants::HIST_FETCHER_FETCH_RESP_SUB_NAME;
use super::entities::{Klines, LatestTradeTime};

#[derive(Debug, Clone)]
pub struct HistoryRecorder {
  col: Collection,
  broker: NatsConnection,
  logger: Logger,
  senders: Vec<mpsc::UnboundedSender<Klines>>,
  stop: broadcast::Sender<()>,
}

impl HistoryRecorder {
  pub fn new(
    col: Collection,
    stop_sender: broadcast::Sender<()>,
    logger: Logger,
    broker: NatsConnection,
  ) -> Self {
    let mut ret = Self {
      col,
      senders: vec![],
      stop: stop_sender,
      broker,
      logger,
    };
    for _ in 0..::num_cpus::get() {
      ret.spawn_record();
    }
    return ret;
  }

  fn spawn_record(&mut self) {
    let (sender, mut recver) = mpsc::unbounded_channel::<Klines>();
    let col = self.col.clone();
    let mut stop = self.stop.subscribe();
    ::tokio::spawn(async move {
      loop {
        select! {
          _ = stop.recv() => {break;},
          raw_klines = recver.recv() => {
            let raw_klines = match raw_klines {
              Some(v) => v,
              None => {break;}
            };
            let klines = block_in_place(move || {
              return raw_klines
                .into_iter()
                .filter_map(|item| to_bson(&item).ok())
                .filter_map(|item| item.as_document().cloned())
                .map(|item| item.clone());
            });
            let _ = col.insert_many(klines, None).await;
          },
        }
      }
    });
    self.senders.push(sender);
  }

  pub async fn spawn(
    &self,
    prog_ch: mpsc::UnboundedSender<SendableErrorResult<HistChartProg>>,
  ) {
    let value_sub = match self
      .broker
      .queue_subscribe(HIST_FETCHER_FETCH_RESP_SUB_NAME, "recorder")
      .await
    {
      Err(e) => {
        crit!(
          self.logger,
          "Failed to subscribe the response channel: {}",
          e; "chan_name" => HIST_FETCHER_FETCH_RESP_SUB_NAME,
        );
        return;
      }
      Ok(v) => v,
    }
    .map(|item| {}); // TODO
    let senders = self.senders.clone();
    let mut stop = self.stop.subscribe();
    ::tokio::spawn(async move {
      let mut counter: usize = 0;
      loop {
        select! {
          _ = stop.recv() => {break;},
          result = value_sub.next() => {
            if let Some(kline_result) = result {
              match kline_result {
                Err(err) => {
                  let _ = prog_ch.send(Err(err));
                  continue;
                }
                Ok(ok) => {
                  let raw_klines = ok.klines;
                  let prog = HistChartProg {
                    symbol: ok.symbol,
                    num_symbols: ok.num_symbols,
                    cur_symbol_num: 1,
                    num_objects: ok.entire_data_len,
                    cur_object_num: 1,
                  };
                  let _ = prog_ch.send(Ok(prog));
                  let _ = senders[counter].send(raw_klines);
                  counter = (counter + 1) % senders.len()
                }
              }
            }
          },
        }
      }
      drop(prog_ch);
    });
  }

  pub async fn get_latest_trade_time(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<HashMap<String, LatestTradeTime<ChronoDateTime<Utc>>>>
  {
    let mut cur = ret_on_err!(
      self
        .col
        .aggregate(
          vec![
            doc! { "$match": doc! { "symbol": doc! { "$in": symbols } } },
            doc! {
              "$group": doc! {
                "_id": "$symbol",
                "open_time": doc! {
                  "$max": "$open_time"
                },
                "close_time": doc! {
                  "$max": "$close_time"
                }
              }
            }
          ],
          None
        )
        .await
    );
    let mut ret = HashMap::new();
    while let Some(doc) = cur.next().await {
      let doc = ret_on_err!(doc);
      let latest: LatestTradeTime<MongoDateTime> =
        ret_on_err!(from_document(doc));
      let latest: LatestTradeTime<ChronoDateTime<Utc>> = latest.into();
      ret.insert(latest.symbol.clone(), latest);
    }
    return Ok(ret);
  }
}
