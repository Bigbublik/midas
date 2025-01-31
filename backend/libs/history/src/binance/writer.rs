use ::std::convert::TryFrom;

use ::async_trait::async_trait;
use ::futures::future::FutureExt;
use ::futures::stream::BoxStream;
use ::futures::StreamExt;
use ::mongodb::bson::{doc, Document};
use ::mongodb::error::Result as MongoResult;
use ::mongodb::results::{DeleteResult, InsertManyResult};
use ::mongodb::{Collection, Database};

use ::errors::WriterResult;
use ::writers::DatabaseWriter;

use super::entities::Kline;
use crate::entities::KlinesByExchange;
use crate::traits::HistoryWriter as HistoryWriterTrait;

#[derive(Debug, Clone)]
pub struct HistoryWriter {
  col: Collection<Kline>,
  db: Database,
}

impl HistoryWriter {
  pub async fn new(db: &Database) -> Self {
    let me = Self {
      col: db.collection("binance.klines"),
      db: db.clone(),
    };
    me.update_indices(&["symbol"]).await;
    return me;
  }
}

#[async_trait]
impl DatabaseWriter for HistoryWriter {
  fn get_database(&self) -> &Database {
    return &self.db;
  }
  fn get_col_name(&self) -> &str {
    return self.col.name();
  }
}

#[async_trait]
impl HistoryWriterTrait for HistoryWriter {
  async fn delete_by_symbol(&self, symbol: &str) -> MongoResult<DeleteResult> {
    return self.col.delete_many(doc! {"symbol": symbol}, None).await;
  }

  async fn write(
    &self,
    klines: KlinesByExchange,
  ) -> WriterResult<InsertManyResult> {
    let klines = Vec::<Kline>::try_from(klines)?;
    return Ok(self.col.insert_many(klines, None).await?);
  }

  async fn list(
    self,
    query: impl Into<Option<Document>> + Send + 'async_trait,
  ) -> MongoResult<BoxStream<'async_trait, KlinesByExchange>> {
    let st = self
      .col
      .find(query, None)
      .map(|cur_res| {
        cur_res.map(|cur| {
          cur
            .filter_map(|kline| async { kline.ok() })
            .map(|kline| KlinesByExchange::Binance(vec![kline]))
            .boxed()
        })
      })
      .await;
    return st;
  }
}
