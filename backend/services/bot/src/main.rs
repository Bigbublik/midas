mod routing;

use ::futures::FutureExt;
use ::log::{info, warn};
use ::warp::Filter;

use ::access_logger::log;
use ::config::init;
use ::csrf::{CSRFOption, CSRF};
use ::handlers::rejection::handle as handle_rejection;

use self::routing::construct;

#[tokio::main]
async fn main() {
  init(|cfg, mut sig, db, _, host| async move {
    let access_logger = log();
    let http_cli = cfg.build_rest_client().unwrap();
    let csrf = CSRF::new(CSRFOption::builder());
    let route = construct(&db, http_cli, &cfg.transpiler_url);
    let route = csrf
      .protect()
      .and(route)
      .with(access_logger)
      .recover(handle_rejection);
    info!("Opened REST server on {}", host);
    let (_, svr) = ::warp::serve(route)
      .tls()
      .cert_path(&cfg.tls.cert)
      .key_path(&cfg.tls.prv_key)
      .bind_with_graceful_shutdown(host, async move {
        sig.recv().await;
      });
    let svr = svr.then(|_| async {
      warn!("REST Server is shutting down! Bye! Bye!");
    });
    svr.await;
  })
  .await;
}
