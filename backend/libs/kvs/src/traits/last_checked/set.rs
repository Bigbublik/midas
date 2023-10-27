use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, ToRedisArgs};

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::base::Set as BaseSet;

use super::base::Base;

#[async_trait]
pub trait Set<T, V>: Base<T> + BaseSet<T, V>
where
  T: Commands + Send,
  for<'async_trait> V: ToRedisArgs + Send + Sync + 'async_trait,
{
  async fn set(
    &self,
    key: Arc<String>,
    value: V,
    opt: Option<WriteOption>,
  ) -> KVSResult<bool> {
    let ret = self.__set__(key.clone(), value, opt.clone()).await?;
    self.flag_last_checked(key, opt.into()).await?;
    return Ok(ret);
  }
}
