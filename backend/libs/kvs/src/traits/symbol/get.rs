use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::errors::KVSResult;
use ::redis::{AsyncCommands as Commands, FromRedisValue};

use super::channel_name::ChannelName;
use crate::traits::base::Base;

#[async_trait]
pub trait Get<T, V>: Base<T> + ChannelName
where
  T: Commands + Send + Sync,
  V: FromRedisValue,
{
  async fn get(
    &self,
    exchange: Arc<String>,
    symbol: Arc<String>,
  ) -> KVSResult<V> {
    let channel_name = self.channel_name(exchange, symbol);
    return Ok(self.__commands__().get(channel_name.as_ref()).await?);
  }
}
