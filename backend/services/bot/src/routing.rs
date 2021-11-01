use ::std::convert::TryFrom;

use ::mongodb::Database;
use ::warp::filters::BoxedFilter;
use ::warp::http::StatusCode;
use ::warp::{Filter, Reply};

use ::bot::entities::Bot;
use ::bot::{BotInfoRecorder, Transpiler};
use ::reqwest::Client;
use ::rpc::bot::Bot as RPCBot;
use ::rpc::entities::Status;

pub fn construct(db: &Database, cli: Client) -> BoxedFilter<(impl Reply,)> {
  let writer = BotInfoRecorder::new(db);
  let register = ::warp::post()
    .and(::warp::filters::body::json())
    .map(move |bot: RPCBot| (bot, cli.clone(), writer.clone()))
    .untuple_one()
    .and_then(
      |bot: RPCBot, cli: Client, writer: BotInfoRecorder| async move {
        let bot = Bot::try_from(bot);
        if let Err(e) = bot {
          let code = StatusCode::EXPECTATION_FAILED;
          let status = Status::new(code.clone(), e.to_string());
          return Err(::warp::reject::custom(status));
        }
        let transpiler = Transpiler::new(cli);
        let bot = transpiler.transpile(&bot.unwrap()).await;
        if let Err(e) = bot {
          let code = StatusCode::INTERNAL_SERVER_ERROR;
          let status = Status::new(code.clone(), e.to_string());
          return Err(::warp::reject::custom(status));
        }
        if let Err(e) = writer.write(&bot.unwrap()).await {
          let code = StatusCode::INTERNAL_SERVER_ERROR;
          let status = Status::new(code.clone(), e.to_string());
          return Err(::warp::reject::custom(status));
        }
        return Ok(());
      },
    )
    .map(|_| {
      return ::warp::reply();
    });
  register.boxed()
}