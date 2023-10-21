use ::std::time::Duration;

use ::async_trait::async_trait;
use ::redis::Commands;

use ::errors::KVSResult;

use crate::traits::base::Expiration as Base;

#[async_trait]
pub trait Expiration<T>: Base<T>
where
  T: Commands + Send,
{
  async fn expire(&self, key: &str, dur: Duration) -> KVSResult<bool> {
    return Base::expire(&self, key, dur).await;
  }
}
