use ::std::collections::HashMap;
use ::std::error::Error;
use ::std::thread;

use ::nats::{Connection as NatsConnection, Subscription as NatsSubsc};

use ::exchanges::Exchange;
use ::rmp_serde::Serializer as MsgPackSer;
use ::serde::Serialize;
use ::slog::{error, o, Logger};
use ::types::{GenericResult, SendableErrorResult};

use crate::entities::KlineFetchStatus;

#[derive(Debug)]
pub(crate) struct ExchangeManager<'nats, T>
where
  T: Exchange + Send,
{
  pub name: String,
  pub exchange: &'nats T,
  nats: &'nats NatsConnection,
  logger: Logger,
}

impl<'nats, T> ExchangeManager<'nats, T>
where
  T: Exchange + Send,
{
  pub fn new(
    name: String,
    exchange: &'nats T,
    nats: &'nats NatsConnection,
    logger: Logger,
  ) -> Self {
    return Self {
      exchange,
      name,
      nats,
      logger,
    };
  }
  pub async fn refresh_historical_klines(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<()> {
    let mut prog = self.exchange.refresh_historical(symbols).await?;
    let logger_in_thread = self
      .logger
      .new(o!("scope" => "refresh_historical_klines.thread"));
    let nats_con = self.nats.clone();
    let name = self.name.clone();
    thread::spawn(move || {
      ::tokio::spawn(async move {
        let mut hist_fetch_prog = HashMap::new();
        loop {
          let prog = match prog.recv().await {
            None => break,
            Some(v) => match v {
              Err(e) => {
                error!(
                  logger_in_thread,
                  "Got an error when getting progress: {}", e
                );
                continue;
              }
              Ok(k) => k,
            },
          };
          let result = match hist_fetch_prog.get_mut(&prog.symbol) {
            None => {
              hist_fetch_prog.insert(prog.symbol.clone(), prog.clone());
              &prog
            }
            Some(v) => {
              v.cur_symbol_num += prog.cur_symbol_num;
              v.cur_object_num += prog.cur_object_num;
              v
            }
          };
          let result = KlineFetchStatus::WIP(result.to_owned());
          nats_broadcast_status(&logger_in_thread, &nats_con, &name, &result);
        }
        let result = KlineFetchStatus::Completed;
        nats_broadcast_status(&logger_in_thread, &nats_con, &name, &result);
      });
    });
    return Ok(());
  }

  pub fn subscribe(&self) -> GenericResult<NatsSubsc> {
    let channel = format!("{}.kline.progress", self.name);
    return match self.nats.subscribe(&channel) {
      Err(err) => Err(Box::new(err)),
      Ok(v) => Ok(v),
    };
  }
}

fn nats_broadcast_status(
  log: &Logger,
  con: &NatsConnection,
  name: &str,
  status: &KlineFetchStatus,
) -> Result<(), Box<dyn Error>> {
  let mut buf: Vec<u8> = Vec::new();
  let msg = match status.serialize(&mut MsgPackSer::new(&mut buf)) {
    Ok(v) => v,
    Err(err) => {
      error!(
        log,
        "Failed to generate a message to broadcast history fetch
                progress: {}, status: {:?}",
        err,
        status,
      );
      return Err(Box::new(err));
    }
  };
  return Ok(con.publish(&format!("{}.kline.progress", name), &buf[..])?);
}
