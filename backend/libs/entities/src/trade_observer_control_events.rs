use ::serde::{Deserialize, Serialize};
use ::uuid::Uuid;

use ::rpc::entities::Exchanges;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverNodeEvent {
  Regist(Uuid, Exchanges),
  Unregist(Uuid),
  Ping(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverControlEvent {
  // First UUID is the old node ID, second UUID is the new node ID.
  NodeIDChanged(Uuid, Uuid),
}
