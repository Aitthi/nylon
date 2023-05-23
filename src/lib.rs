// mod
pub mod env;
pub mod handler;
pub mod logger;
pub mod nylon;
pub mod router;
pub mod server;

/*
// use
use hyper::Server;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time;
use tower::{make::Shared, ServiceBuilder};
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};

#[napi]
pub async fn listen(
  port: u16,
  host: String,
  callback: ThreadsafeFunction<()>,
  routes: HashMap<String, Vec<router::Handler>>,
) -> Result<bool> {
  // Setup tracing
  tracing_subscriber::fmt::init();
  tracing::info!("Starting Nylon application...");
  let app_start_time = time::Instant::now();
  let span = tracing::span!(tracing::Level::INFO, "routes");
  let mut registers = router::Router::new();
  for route in routes.iter() {
    let now = time::Instant::now();
    let _ = registers.delegate(route.0.as_str(), route.1.clone());
    tracing::info!(
      parent: &span,
      "Mapped {} +{}us",
      route.0.as_str(),
      now.elapsed().as_micros()
    );
  }
  let mut host = host;
  if host == "localhost" {
    host = "127.0.0.1".to_string();
  }
  let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
  let init_service = ServiceBuilder::new()
    .layer(TraceLayer::new_for_http())
    .layer(AddExtensionLayer::new(registers))
    .service_fn(service::handler);
  let server = Server::bind(&addr).serve(Shared::new(init_service));
  callback.call(Ok(()), ThreadsafeFunctionCallMode::NonBlocking);
  tracing::info!(
    "Nylon application successfully started +{}ms",
    app_start_time.elapsed().as_millis()
  );
  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  }
  tokio::task::spawn(async { Ok(true) }).await.unwrap()
}
*/
