use ::std::collections::HashMap;

use ::async_trait::async_trait;

use ::mongodb::Database;
use ::slog::{o, Logger};
use ::tokio::sync::{broadcast, mpsc};
use ::tonic::{Request, Response};

use ::types::{Result, SendableErrorResult};

use ::rpc::historical::{
  hist_chart_server::HistChart, HistChartFetchReq, HistChartProg, Status,
};

use ::exchanges::{Binance, Exchange};

#[derive(Debug)]
struct ExchangeManager<T>
where
  T: Exchange,
{
  pub exchange: T,
  pub hist_fetch_prog: HashMap<String, HistChartProg>,
}

impl<T> ExchangeManager<T>
where
  T: Exchange,
{
  fn new(exchange: T) -> Self {
    return Self {
      exchange,
      hist_fetch_prog: HashMap::new(),
    };
  }
  async fn refresh_historical_klines(
    &mut self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<(broadcast::Sender<()>, mpsc::Receiver<HistChartProg>)>
  {
    let hist_fut = self.exchange.refresh_historical(symbols);
    return hist_fut.await;
  }
}

#[derive(Debug)]
pub struct Server {
  logger: Logger,
  binance: ExchangeManager<Binance>,
}

impl Server {
  fn new(log: Logger, db: &Database) -> Self {
    return Self {
      logger: log,
      binance: ExchangeManager::new(Binance::new(
        log.new(o!("Exchange" => "Binance")),
        db.collection("binance.history"),
        db.collection("binance.symbolinfo"),
      )),
    };
  }
}

#[async_trait]
impl HistChart for Server {
  type syncStream = mpsc::Receiver<Result<HistChartProg>>;

  async fn sync(
    &self,
    req: Request<HistChartFetchReq>,
  ) -> Result<Response<Self::syncStream>> {
    let manager = self.binance;
    let res = manager.refresh_historical_klines(req.symbols).await?;
  }
}
