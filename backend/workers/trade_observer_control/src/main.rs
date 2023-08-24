mod dlock;
mod errors;
mod handler;

use ::futures::StreamExt;
use ::log::{error, info};
use ::tokio::select;

use ::observers::kvs::{ObserverNodeKVS, ObserverNodeLastCheckKVS};
use ::observers::pubsub::NodeEventPubSub;
use ::subscribe::PubSub;

use ::config;

#[tokio::main]
async fn main() {
  info!("Starting trade_observer_control");
  config::init(|cfg, mut sig, db, broker, host| async move {
    let kvs = cfg.redis().unwrap();
    let node_event_pubsub = NodeEventPubSub::new(&broker).await.unwrap();
    let mut node_event_handler = handler::FromNodeEventHandler::new(kvs);
    let mut node_event = node_event_pubsub
      .queue_subscribe("tradeObserverController")
      .await
      .unwrap();
    loop {
      select! {
        event = node_event.next() => if let Some((event, _)) = event {
          if let Err(e) = node_event_handler.handle(event) {
            error!("Error handling node event: {}", e);
          }
        },
        _ = sig.recv() => {
          break;
        },
      };
    }
  })
  .await;
  info!("Stopping trade_observer_control");
}
