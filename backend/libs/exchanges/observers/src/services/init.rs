use ::std::fmt::Debug;
use ::std::sync::Arc;

use ::futures::future::{try_join_all, FutureExt};

use ::config::Database;
use ::errors::{ObserverError, ObserverResult};
use ::kvs::redis::AsyncCommands as Commands;
use ::kvs::traits::normal::Lock;
use ::log::{as_serde, info};
use ::rpc::exchanges::Exchanges;
use ::subscribe::nats::Client as Nats;
use ::subscribe::PubSub;

use crate::kvs::INIT_LOCK_BUILDER;

use crate::pubsub::NodeControlEventPubSub;

use super::NodeDIffTaker;
use super::ObservationBalancer;

#[derive(Clone)]
pub struct Init<C>
where
  C: Commands + Clone + Sync + Send + Debug + 'static,
{
  diff_taker: Arc<NodeDIffTaker<C>>,
  balancer: Arc<ObservationBalancer<C>>,
  control_pubsub: NodeControlEventPubSub,
  dlock: Arc<dyn Lock<Commands = C, Value = ObserverResult<()>> + Send + Sync>,
}

impl<C> Init<C>
where
  C: Commands + Clone + Sync + Send + Debug + 'static,
{
  pub async fn new(
    kvs: C,
    db: Database,
    nats: &Nats,
  ) -> ObserverResult<Init<C>> {
    let diff_taker =
      Arc::new(NodeDIffTaker::new(&db, kvs.clone().into()).await?);
    let balancer =
      Arc::new(ObservationBalancer::new(kvs.clone().into()).await?);
    let control_pubsub = NodeControlEventPubSub::new(nats).await?;
    let dlock = Arc::new(INIT_LOCK_BUILDER.build(kvs));

    return Ok(Self {
      diff_taker,
      balancer,
      control_pubsub,
      dlock,
    });
  }

  pub async fn init(&self, exchange: Box<Exchanges>) -> ObserverResult<()> {
    let me = self.clone();
    let _ = self
      .dlock
      .lock(exchange.to_string().into(), Box::pin(move || {
        let me = me.clone();
        let exchange = exchange.clone();
        async move {
          let diff = me.diff_taker.get_symbol_diff(exchange.clone()).await?;
          let balanced = me.balancer.get_event_to_balancing(exchange).await?;
          let controls_to_publish = &diff | &balanced;
          info!(events = as_serde!(controls_to_publish); "Publishing symbol control events.");
          let defer: Vec<_> = controls_to_publish
            .into_iter()
            .map(|event| me.control_pubsub.publish(event))
            .collect();
          let _ = try_join_all(defer).await?;
          return Ok::<(), ObserverError>(());
        }.boxed()
      }))
      .await?;
    return Ok(());
  }
}
