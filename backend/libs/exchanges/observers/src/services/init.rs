use ::futures::future::try_join_all;

use ::config::Database;
use ::errors::{ObserverError, ObserverResult};
use ::kvs::redis::Commands;
use ::kvs::traits::normal::Lock;
use ::kvs::Connection;
use ::log::{as_serde, info};
use ::rpc::entities::Exchanges;
use ::subscribe::nats::Client as Nats;
use ::subscribe::PubSub;

use crate::kvs::InitLock;
use crate::pubsub::NodeControlEventPubSub;

use super::NodeDIffTaker;
use super::ObservationBalancer;

pub struct Init<C>
where
  C: Commands + Sync + Send,
{
  diff_taker: NodeDIffTaker<C>,
  balancer: ObservationBalancer<C>,
  control_pubsub: NodeControlEventPubSub,
  dlock: InitLock<C>,
}

impl<C> Init<C>
where
  C: Commands + Sync + Send,
{
  pub async fn new(
    kvs: Connection<C>,
    db: Database,
    nats: &Nats,
  ) -> ObserverResult<Self> {
    let diff_taker = NodeDIffTaker::new(&db, kvs.clone().into()).await?;
    let balancer = ObservationBalancer::new(kvs.clone().into()).await?;
    let control_pubsub = NodeControlEventPubSub::new(nats).await?;
    let dlock = InitLock::new(kvs.into());

    return Ok(Self {
      diff_taker,
      balancer,
      control_pubsub,
      dlock,
    });
  }

  pub async fn init(&self, exchange: Exchanges) -> ObserverResult<()> {
    let _ = self
      .dlock
      .lock(exchange.as_str_name(), || async move {
        let diff = self.diff_taker.get_symbol_diff(&exchange).await?;
        let balanced = self.balancer.get_event_to_balancing(exchange).await?;
        let controls_to_publish = &diff | &balanced;
        info!(events = as_serde!(controls_to_publish); "Publishing symbol control events.");
        let defer: Vec<_> = controls_to_publish
          .into_iter()
          .map(|event| self.control_pubsub.publish(event))
          .collect();
        let _ = try_join_all(defer).await?;
        return Ok::<(), ObserverError>(());
      })
      .await?;
    return Ok(());
  }
}