use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue};

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Remove<T>: Base<T> + ChannelName
where
  T: Commands + Send,
{
  async fn del<R>(&self, key: impl AsRef<str> + Send + Display) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let mut cmd = self.commands().lock().await;
    let channel_name = self.channel_name(key);
    return Ok(cmd.del(channel_name)?);
  }
}
