use ::std::time::{Duration, UNIX_EPOCH};

use ::clap::Parser;
use ::futures::StreamExt;
use ::libc::{SIGINT, SIGTERM};
use ::slog::{error, info};
use ::tokio::select;
use ::tokio::signal::unix as signal;

use ::config::{CmdArgs, Config};
use ::date_splitter::DateSplitter;
use ::history::kvs::{CurrentSyncProgressStore, NumObjectsToFetchStore};
use ::history::pubsub::{HistChartDateSplitPubSub, HistChartPubSub};
use ::history::traits::Store as StoreTrait;
use ::rpc::entities::Exchanges;
use ::subscribe::PubSub;

#[tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  let logger = cfg.build_slog();
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();

  let mut cur_prog_kvs =
    CurrentSyncProgressStore::new(cfg.redis(&logger).unwrap());
  let mut num_prg_kvs =
    NumObjectsToFetchStore::new(cfg.redis(&logger).unwrap());
  let broker = cfg.nats_cli().unwrap();

  let req_pubsub = HistChartDateSplitPubSub::new(broker.clone());
  let mut req_sub =
    req_pubsub.queue_subscribe("histChartDateSplitter").unwrap();
  let resp_pubsub = HistChartPubSub::new(broker.clone());

  info!(logger, "Ready...");
  loop {
    select! {
      Some((req, _)) = req_sub.next() => {
        let start = req.start.map(|start| start.into()).unwrap_or(UNIX_EPOCH);
        let end = req.end.map(|end| end.into()).unwrap_or(UNIX_EPOCH);
        info!(
          logger,
          "Start splitting currency {:?} from {:?} to {:?}",
          req.symbol, start, end,
        );
        let splitter = match req.exchange {
          Exchanges::Binance => {
            DateSplitter::new(start, end, Duration::from_secs(60))
          },
        };
        let mut splitter = match splitter {
          Err(e) => {
            error!(logger, "Failed to initialize DateSplitter: {:?}", e);
            continue;
          },
          Ok(v) => v
        };
        if let Err(e) = cur_prog_kvs.reset(req.exchange.as_string(), &req.symbol) {
          error!(logger, "Failed to reset the progress: {:?}", e);
          continue;
        }
        if let Err(e) = num_prg_kvs.set(
          req.exchange.as_string(),
          &req.symbol,
          splitter.len().unwrap_or(0) as i64
        ) {
          error!(logger, "Failed to set the number of objects to fetch: {:?}", e);
        }
        while let Some((start, end)) = splitter.next().await {
          let resp = req.clone().start(Some(start.into())).end(Some(end.into()));
          let _ = resp_pubsub.publish(&resp);
          info!(
            logger,
            "Sent the split date data (start: {:?}, end: {:?})",
            start, end
          );
        }
      },
      _ = sig.recv() => {break;},
    }
  }
}
