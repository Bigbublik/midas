use ::std::pin::Pin;

use ::futures::Stream;

use ::futures::future::join_all;
use ::mongodb::Database;
use ::nats::Connection as NatsCon;
use ::num_traits::FromPrimitive;
use ::rmp_serde::from_slice as read_msgpack;
use ::slog::{error, o, Logger};
use ::tonic::{async_trait, Code, Request, Response, Status};

use ::exchanges::{binance, HistoryFetcher};
use ::rpc::entities::Exchanges;
use ::rpc::historical::{
  hist_chart_server::HistChart, HistChartFetchReq, HistChartProg, StopRequest,
};
use ::types::{rpc_ret_on_err, GenericResult, Result};

use super::manager::ExchangeManager;

#[derive(Debug)]
pub struct Service {
  logger: Logger,
  binance: binance::HistoryFetcher,
  nats: NatsCon,
}

impl Service {
  pub fn new(
    log: &Logger,
    db: &Database,
    nats: NatsCon,
  ) -> GenericResult<Self> {
    let log = log.new(o!("scope" => "History Fetch RPC Service"));
    return Ok(Self {
      logger: log.clone(),
      binance: binance::HistoryFetcher::new(
        None,
        db.collection("binance.history"),
        log.new(o!("exchange" => "Binance", "scope" => "HistoryFetch")),
        nats.clone(),
        binance::SymbolFetcher::new(
          log.new(o!("exchange" => "Binance", "scope" => "SymbolFetch")),
          db.collection("binance.symbolinfo"),
        ),
      )?,
      nats,
    });
  }
}

#[async_trait]
impl HistChart for Service {
  async fn sync(
    &self,
    req: Request<HistChartFetchReq>,
  ) -> Result<Response<()>> {
    let req = req.into_inner();
    let manager = ExchangeManager::new(
      String::from("binance"),
      &self.binance,
      &self.nats,
      self.logger.new(o!("scope" => "Binance Exchange Manager")),
    );
    rpc_ret_on_err!(
      Code::Internal,
      manager.refresh_historical_klines(req.symbols).await
    );
    return Ok(Response::new(()));
  }

  type subscribeStream =
    Pin<Box<dyn Stream<Item = Result<HistChartProg>> + Send + Sync + 'static>>;
  async fn subscribe(
    &self,
    _: tonic::Request<()>,
  ) -> Result<tonic::Response<Self::subscribeStream>> {
    let manager = ExchangeManager::new(
      String::from("binance"),
      &self.binance,
      &self.nats,
      self.logger.new(o!("scope" => "Binance Exchange Manager")),
    );
    let subscriber = rpc_ret_on_err!(Code::Internal, manager.subscribe());
    let stream_logger = self.logger.new(o!("scope" => "Stream Logger"));
    let out = ::async_stream::try_stream! {
      while let Some(msg) = subscriber.next() {
        let prog: HistChartProg = match read_msgpack(&msg.data[..]) {
          Err(e) => {
            error!(
              stream_logger,
              "Got an error while deserializing HistFetch Prog. {}",
              e
            );
            continue;
          },
          Ok(v) => v,
        };
        yield prog;
      }
    };
    return Ok(Response::new(Box::pin(out) as Self::subscribeStream));
  }

  async fn stop(
    &self,
    request: tonic::Request<StopRequest>,
  ) -> Result<tonic::Response<()>> {
    let req = request.into_inner();
    let mut stop_vec = vec![];
    for exc in req.exchanges {
      match FromPrimitive::from_i32(exc) {
        Some(Exchanges::Binance) => {
          stop_vec.push(self.binance.clone().stop());
        }
        _ => {
          continue;
        }
      }
    }
    for result in join_all(stop_vec).await {
      rpc_ret_on_err!(Code::Internal, result);
    }
    return Ok(Response::new(()));
  }
}
