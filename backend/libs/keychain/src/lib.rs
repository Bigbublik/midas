pub mod pubsub;

use ::futures::stream::BoxStream;
use ::futures::StreamExt;
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::bson::{doc, from_document, to_document, Document};
use ::mongodb::error::Result;
use ::mongodb::options::UpdateModifications;
use ::mongodb::{Collection, Database};
use ::nats::Connection as NatsCon;
use ::subscribe::PubSub;

use ::rpc::entities::Exchanges;
use ::types::{GenericResult, ThreadSafeResult};

use ::base_recorder::Recorder;
pub use ::entities::APIKey;
use ::entities::APIKeyEvent;

use self::pubsub::APIKeyPubSub;

#[derive(Debug, Clone)]
pub struct KeyChain {
  pubsub: APIKeyPubSub,
  db: Database,
  col: Collection,
}

impl KeyChain {
  pub async fn new(broker: NatsCon, db: Database) -> Self {
    let col = db.collection("apiKeyChains");
    let ret = Self {
      pubsub: APIKeyPubSub::new(broker),
      db,
      col,
    };
    ret.update_indices(&["exchange"]).await;
    return ret;
  }

  pub async fn push(&self, api_key: APIKey) -> GenericResult<Option<ObjectId>> {
    let value = to_document(&api_key)?;
    let result = self.col.insert_one(value.to_owned(), None).await?;
    let id = result.inserted_id.as_object_id();
    let mut api_key = api_key.clone();
    api_key.inner_mut().id = id.cloned();
    let event = APIKeyEvent::Add(api_key);
    let _ = self.pubsub.publish(&event)?;
    return Ok(id.cloned());
  }

  pub async fn rename_label(
    &self,
    id: ObjectId,
    label: &str,
  ) -> GenericResult<()> {
    let _ = self
      .col
      .update_one(
        doc! { "_id": id },
        UpdateModifications::Pipeline(vec![doc! {
          "$set": doc! {"label": label},
        }]),
        None,
      )
      .await?;
    return Ok(());
  }

  pub async fn list(
    &self,
    filter: Document,
  ) -> ThreadSafeResult<BoxStream<'_, APIKey>> {
    let stream = self
      .col
      .find(filter, None)
      .await?
      .filter_map(|res| async { res.ok() })
      .map(|doc| from_document::<APIKey>(doc))
      .filter_map(|ent| async { ent.ok() })
      .boxed();
    return Ok(stream);
  }

  pub async fn get(
    &self,
    exchange: Exchanges,
    id: ObjectId,
  ) -> Result<Option<APIKey>> {
    let key = self
      .col
      .find_one(
        doc! {
          "_id": id,
          "exchange": exchange.as_string()
        },
        None,
      )
      .await?
      .map(|k| from_document::<APIKey>(k).ok())
      .flatten();
    return Ok(key);
  }

  pub async fn delete(&self, id: ObjectId) -> GenericResult<()> {
    if let Some(doc) =
      self.col.find_one_and_delete(doc! {"_id": id}, None).await?
    {
      let api_key: APIKey = from_document(doc)?;
      let event = APIKeyEvent::Remove(api_key);
      let _ = self.pubsub.publish(&event)?;
    }
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
