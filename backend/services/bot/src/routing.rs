use ::std::convert::TryFrom;
use ::std::sync::Arc;

use ::futures::TryFutureExt;
use ::http::StatusCode;
use ::warp::filters::BoxedFilter;
use ::warp::{Filter, Rejection, Reply};

use ::bot::entities::Bot;
use ::rpc::bot::Bot as RPCBot;
use ::rpc::pagination::Pagination;
use ::rpc::status::Status;

use crate::context::Context;

pub fn construct(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  return post(ctx.clone()).or(put(ctx)).boxed();
}

fn post(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  let register = ::warp::post()
    .and(::warp::filters::body::json())
    .map(move |bot: RPCBot| (bot, ctx.clone()))
    .untuple_one()
    .and_then(|bot: RPCBot, ctx: Arc<Context>| async move {
      let mut bot = Bot::try_from(bot).map_err(|e| {
        let code = StatusCode::EXPECTATION_FAILED;
        let status = Status::new(code.clone(), &e.to_string());
        return ::warp::reject::custom(status);
      })?;
      bot = ctx
        .transpiler
        .transpile(bot)
        .map_err(|e| {
          let code = StatusCode::INTERNAL_SERVER_ERROR;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })
        .await?;
      let _ = ctx
        .bot_repo
        .save(&[&bot])
        .map_err(|e| {
          let code = StatusCode::INTERNAL_SERVER_ERROR;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })
        .await?;
      return Ok::<Bot, Rejection>(bot);
    })
    .map(|bot: Bot| {
      let bot: RPCBot = bot.into();
      return ::warp::reply::json(&bot);
    });
  return register.boxed();
}

fn put(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  let modify = ::warp::path::param()
    .and(::warp::put())
    .and(::warp::filters::body::json())
    .and_then(|id: String, bot: RPCBot| async move {
      // Check ID, and then convert RPCBot to Bot that is used in the backend.
      if Some(id) == bot.id {
        let bot = Bot::try_from(bot).map_err(|e| {
          let code = StatusCode::EXPECTATION_FAILED;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })?;
        return Ok(bot);
      }
      let code = StatusCode::EXPECTATION_FAILED;
      let status = Status::new(code.clone(), "ID mismatch");
      return Err(::warp::reject::custom(status));
    })
    .map(move |bot: Bot| (bot, ctx.clone()))
    .untuple_one()
    .and_then(|bot: Bot, ctx: Arc<Context>| async move {
      let _ = ctx
        .bot_repo
        .save(&[&bot])
        .map_err(|e| {
          let code = StatusCode::INTERNAL_SERVER_ERROR;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })
        .await?;
      return Ok::<Bot, Rejection>(bot);
    })
    .map(|bot: Bot| {
      let bot: RPCBot = bot.into();
      return ::warp::reply::json(&bot);
    });
  return modify.boxed();
}

fn list(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  let list = ::warp::get()
    .and(::warp::path::end())
    .and(::warp::query::<Pagination>())
    .and_then(|pagination: Pagination| async move {
      let stream = ctx
        .bot_repo
        .list(pagination.offset, pagination.limit)
        .await?;
    });
}
