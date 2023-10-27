mod filter;
mod indexer;

use ::kvs::{LastCheckedKVSBuilder, NormalKVSBuilder};

pub use self::filter::NodeFilter;
pub use self::indexer::NodeIndexer;

pub const NODE_KVS_BUILDER: LastCheckedKVSBuilder<String> =
  LastCheckedKVSBuilder::new("observer_node");

pub const NODE_EXCHANGE_TYPE_KVS_BUILDER: LastCheckedKVSBuilder<String> =
  LastCheckedKVSBuilder::new("observer_node_exchange_type");

pub const INIT_LOCK_BUILDER: NormalKVSBuilder<String> =
  NormalKVSBuilder::<String>::new("init_lock");
