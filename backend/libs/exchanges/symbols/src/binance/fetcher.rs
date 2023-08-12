use ::std::time::Duration as StdDur;

use ::async_trait::async_trait;
use ::futures::future::join;
use ::futures::stream::StreamExt;
use ::mongodb::bson::{doc, Document};
use ::mongodb::error::Result as DBResult;
use ::mongodb::Database;
use ::nats::jetstream::JetStream as NatsJS;
pub use ::reqwest::Result as ReqRes;
use ::url::Url;

use ::clients::binance::REST_ENDPOINTS;
use ::errors::{SymbolFetchError, SymbolFetchResult};
use ::round::RestClient;
use ::rpc::symbols::SymbolInfo;

use super::entities::{ExchangeInfo, Symbol};
use super::manager::SymbolUpdateEventManager;
use super::recorder::SymbolWriter;

use crate::traits::SymbolFetcher as SymbolFetcherTrait;
use crate::traits::SymbolWriter as SymbolWriterTrait;
use ::errors::StatusFailure;

#[derive(Debug, Clone)]
pub struct SymbolFetcher {
  broker: NatsJS,
  recorder: SymbolWriter,
  cli: RestClient,
}

impl SymbolFetcher {
  pub async fn new(broker: NatsJS, db: &Database) -> ReqRes<Self> {
    let recorder = SymbolWriter::new(&db).await;
    let urls: Vec<Url> = REST_ENDPOINTS
      .into_iter()
      .filter_map(|&url| {
        (String::from(url) + "/api/v3/exchangeInfo").parse().ok()
      })
      .collect();
    let ret = Self {
      broker: broker,
      cli: RestClient::new(urls, StdDur::from_secs(5), StdDur::from_secs(5))?,
      recorder,
    };
    return Ok(ret);
  }

  pub async fn get(
    &self,
    filter: impl Into<Option<Document>> + Send,
  ) -> DBResult<Vec<SymbolInfo>> {
    let docs = self.recorder.list(filter).await?;
    let docs: Vec<SymbolInfo> = docs.map(|doc| doc.into()).collect().await;
    return Ok(docs);
  }
}

#[async_trait]
impl SymbolFetcherTrait for SymbolFetcher {
  type SymbolType = Symbol;
  async fn refresh(&mut self) -> SymbolFetchResult<Vec<Self::SymbolType>> {
    let resp = self.cli.get::<()>(None, None).await?;
    let old_symbols = self.recorder.list(doc! {}).await?;
    let old_symbols: Vec<Symbol> = old_symbols.collect().await;
    let resp_status = resp.status();
    if resp_status.is_success() {
      let info: ExchangeInfo = resp.json().await?;
      let new_symbols = info.symbols.clone();
      let update_event_manager = SymbolUpdateEventManager::new(
        self.broker.clone(),
        new_symbols.clone(),
        old_symbols,
      )?;
      let (update, _) = join(
        self.recorder.update_symbols(new_symbols),
        update_event_manager.publish_changes(),
      )
      .await;
      update?;
      return Ok(self.recorder.list(None).await?.collect().await);
    } else {
      return Err(SymbolFetchError::HTTPErr(
        StatusFailure {
          url: Some(self.cli.get_current_url()).cloned(),
          code: resp_status.as_u16(),
          text: resp.text().await?,
        }
        .into(),
      ));
    }
  }
}
