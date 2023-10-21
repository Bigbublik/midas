use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::base::Set as BaseSet;

use super::base::Base;

#[async_trait]
pub trait Set<T, V>: Base<T> + BaseSet<T, V>
where
  T: Commands + Send,
  for<'a> V: ToRedisArgs + Send + 'a,
{
  async fn set<R>(
    &self,
    key: &str,
    value: V,
    opt: Option<WriteOption>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue + Send,
  {
    let ret = BaseSet::set(self, &key, value, opt.clone()).await?;
    self.flag_last_checked(key, opt.into()).await?;
    return Ok(ret);
  }
}
