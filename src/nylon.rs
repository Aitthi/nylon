use std::{collections::HashMap, time};

use napi::{bindgen_prelude::*, threadsafe_function::ThreadsafeFunction};
use napi_derive::napi;

use crate::{
  router::{self, Router},
  server,
};

#[napi]
pub struct Nylon {
  routes: Router,
}

#[napi]
impl Nylon {
  #[napi]
  pub fn init() -> Self {
    // Setup tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Nylon application...");
    Nylon {
      routes: router::Router::new(),
    }
  }

  #[napi]
  pub async fn http(
    &self,
    port: u16,
    host: String,
    callback: ThreadsafeFunction<()>,
  ) -> Result<bool> {
    callback.call(
      Ok(()),
      napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
    );
    server::http::run(port, host, self.routes.clone())
      .await
      .unwrap();
    Ok(true)
  }

  #[napi]
  pub fn add_route(&mut self, routes: HashMap<String, router::Handler>) -> Result<bool> {
    let span = tracing::span!(tracing::Level::INFO, "Routes");
    for route in routes.iter() {
      let now = time::Instant::now();
      let _ = self.routes.delegate(route.0.as_str(), route.1.clone());
      tracing::info!(
        parent: &span,
        "Mapped {} +{}us",
        route.0.as_str(),
        now.elapsed().as_micros()
      );
    }
    Ok(true)
  }
}
