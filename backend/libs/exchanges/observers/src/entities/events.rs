use ::serde::{Deserialize, Serialize};

use ::rpc::entities::Exchanges;
use ::symbols::entities::SymbolEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverNodeEvent {
  /// Regist notifies a node is registered/
  Regist(Exchanges),
  /// Unregist notifies a node is unregistered.
  ///
  /// ### Values
  ///
  /// Exchanges: the exchanges the node was registered.
  ///
  /// Vec<String>: the symbols the node was registered.
  Unregist(Exchanges, Vec<String>),
  Ping(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverControlEvent {
  /// Triggered when the controller instructs the observer to add a symbol.
  SymbolAdd(Exchanges, String),
  /// Triggered when the controller instructs the observer to remove a symbol.
  SymbolDel(Exchanges, String),
}

impl From<SymbolEvent> for TradeObserverControlEvent {
  fn from(value: SymbolEvent) -> Self {
    return match value {
      SymbolEvent::Add(info) => {
        TradeObserverControlEvent::SymbolAdd(info.exchange(), info.symbol)
      }
      SymbolEvent::Remove(info) => {
        TradeObserverControlEvent::SymbolDel(info.exchange(), info.symbol)
      }
    };
  }
}