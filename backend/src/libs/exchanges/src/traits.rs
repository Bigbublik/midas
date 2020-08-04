use ::async_trait::async_trait;
use ::mongodb::bson::Document;
use ::rpc::entities::SymbolInfo;
use ::rpc::historical::HistChartProg;
use ::tokio::sync::{mpsc, oneshot};
use ::types::SendableErrorResult;

#[async_trait]
pub trait Exchange {
  async fn refresh_historical(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<(oneshot::Sender<()>, mpsc::Receiver<HistChartProg>)>;
  async fn get_symbols(
    &self,
    filter: impl Into<Option<Document>> + Send + 'async_trait,
  ) -> SendableErrorResult<Vec<SymbolInfo>>;
  async fn refresh_symbols(self) -> SendableErrorResult<()>;
}