use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};
use ::std::fmt::Display;
use ::std::num::NonZeroUsize;

use ::errors::{KVSError, KVSResult};

use super::{Base, Exist, Lock};

use crate::options::WriteOptionTrait;
use crate::WriteOption;

#[async_trait]
pub trait ListOp<T, V>: Base<T> + Lock<T> + Exist<T>
where
  T: Commands + Send,
  for<'a> V: FromRedisValue + ToRedisArgs + Send + 'a,
{
  async fn lpush<R>(
    &self,
    key: impl AsRef<str> + Clone + Display + Send + Sync,
    value: V,
    opt: impl Into<Option<WriteOption>> + Send,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(&key);
    let opt: Option<WriteOption> = opt.into();

    let key_exists = self.exists(&key).await;
    let cmds = self.commands();
    let mut cmds = cmds.lock().await;
    let res = if opt.non_existent_only() {
      match key_exists {
        Ok(exists) => {
          if exists {
            return Err(KVSError::KeyExists(key.to_string()));
          } else {
            cmds.lpush(&channel_name, value)?
          }
        }
        Err(e) => return Err(e),
      }
    } else {
      cmds.lpush(&channel_name, value)?
    };

    opt.execute(&mut cmds, &channel_name)?;
    return Ok(res);
  }

  async fn lpop(
    &self,
    key: impl AsRef<str> + Display + Send,
    count: Option<NonZeroUsize>,
  ) -> KVSResult<V> {
    let channel_name = self.channel_name(key);
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.lpop(channel_name, count)?);
  }

  async fn lrange<R>(
    &self,
    key: impl AsRef<str> + Display + Send,
    start: isize,
    stop: isize,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(key);
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.lrange(channel_name, start, stop)?);
  }

  async fn llen<R>(&self, key: impl AsRef<str> + Display + Send) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(key);
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.llen(channel_name)?);
  }
}
