use crate::handler;
use crate::router::Router;
use hyper::Server;
use std::net::SocketAddr;
use tower::make::Shared;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::TraceLayer;

// #[napi]
pub async fn run(port: u16, host: String, routes: Router) -> anyhow::Result<Option<bool>> {
  let mut host = host;
  if host == "localhost" {
    host = "127.0.0.1".to_string();
  }
  let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();
  let init_service = ServiceBuilder::new()
    .layer(TraceLayer::new_for_http())
    .layer(AddExtensionLayer::new(routes))
    .service_fn(handler::request);
  let server = Server::bind(&addr).serve(Shared::new(init_service));
  let _ = server.await;
  Ok(Some(true))
}
