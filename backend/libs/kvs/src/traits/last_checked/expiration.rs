use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::base::Expiration as BaseExp;

use super::base::Base;

#[async_trait]
pub trait Expiration<T>: Base<T> + BaseExp<T>
where
  T: Commands + Send,
{
  async fn expire(&self, key: Arc<String>, dur: Duration) -> KVSResult<bool> {
    let ret = self.__expire__(key.clone(), dur).await?;
    self
      .flag_last_checked(
        key,
        WriteOption::default().duration(dur.into()).into(),
      )
      .await?;
    return Ok(ret);
  }
}
