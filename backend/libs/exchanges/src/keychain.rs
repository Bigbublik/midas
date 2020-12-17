use ::futures::stream::BoxStream;
use ::futures::StreamExt;
use ::mongodb::bson::oid::{ObjectId, Result};
use ::mongodb::bson::{from_document, to_document, Document};
use ::mongodb::{Collection, Database};

use ::types::{ret_on_err, GenericResult, SendableErrorResult};

use crate::entities::APIKey;
use crate::traits::Recorder;

#[derive(Debug, Clone)]
pub struct KeyChain {
  db: Database,
  col: Collection,
}

impl KeyChain {
  pub async fn new(db: Database) -> Self {
    let col = db.collection("apiKeyChains");
    let ret = Self { db, col };
    ret.update_indices(&["exchange"]).await;
    return ret;
  }

  pub async fn write(&self, value: APIKey<String>) -> GenericResult<()> {
    let value: Result<APIKey<ObjectId>> = value.into();
    let value = value?;
    let value = to_document(&value)?;
    let _ = self.col.insert_one(value, None).await?;
    return Ok(());
  }

  pub async fn list(
    &self,
    filter: Document,
  ) -> SendableErrorResult<BoxStream<'_, APIKey<String>>> {
    let stream = ret_on_err!(self.col.find(filter, None).await)
      .filter_map(|res| async { res.ok() })
      .map(|doc| from_document::<APIKey<ObjectId>>(doc))
      .filter_map(|ent| async { ent.ok() })
      .map(|api| api.into())
      .boxed();
    return Ok(stream);
  }

  pub async fn delete(&self, query: Document) -> GenericResult<()> {
    self.col.delete_many(query, None).await?;
    return Ok(());
  }
}

impl Recorder for KeyChain {
  fn get_database(&self) -> &Database {
    return &self.db;
  }

  fn get_col_name(&self) -> &str {
    return self.col.name();
  }
}
