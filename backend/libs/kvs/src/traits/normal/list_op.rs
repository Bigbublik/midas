use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::{FromRedisValue, ToRedisArgs};
use ::std::num::NonZeroUsize;

use ::errors::KVSResult;

use crate::traits::base::ListOp as Base;

use crate::WriteOption;

#[async_trait]
pub trait ListOp: Base {
  async fn lpush(
    &self,
    key: Arc<String>,
    value: Vec<Arc<dyn ToRedisArgs>>,
    opt: impl Into<Option<WriteOption>> + Send,
  ) -> KVSResult<usize> {
    return self.__lpush__(key, value, opt).await;
  }

  async fn lpop(
    &self,
    key: Arc<String>,
    count: Option<NonZeroUsize>,
  ) -> KVSResult<Arc<dyn FromRedisValue>> {
    return self.__lpop__(key, count).await;
  }

  async fn lrem(
    &self,
    key: Arc<String>,
    count: isize,
    elem: Arc<dyn ToRedisArgs>,
  ) -> KVSResult<usize> {
    return self.__lrem__(key, count, elem).await;
  }

  async fn lrange(
    &self,
    key: Arc<String>,
    start: isize,
    stop: isize,
  ) -> KVSResult<Vec<Arc<dyn FromRedisValue>>> {
    return self.__lrange__(key, start, stop).await;
  }

  async fn llen(&self, key: Arc<String>) -> KVSResult<usize> {
    return self.__llen__(key).await;
  }
}
