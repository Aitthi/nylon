use axum::{
  body::Body,
  extract::Request,
  http::{Response, StatusCode},
  Router,
};
use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy::Fatal, ThreadsafeFunction},
};
use napi_derive::napi;
use serde_json::Value;
use std::sync::Mutex;

#[napi]
pub struct Nylon {
  router: Mutex<Router>,
}

#[napi]
impl Nylon {
  #[napi(constructor)]
  pub fn new() -> Self {
    // Setup tracing
    tracing::info!("Starting Nylon application...");
    Nylon {
      router: Mutex::new(Router::new()),
    }
  }

  #[napi]
  pub async fn listen(
    &self,
    port: u16,
    host: Option<String>,
    callback: ThreadsafeFunction<()>,
  ) -> Result<bool> {
    callback.call(
      Ok(()),
      napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
    );
    // Start the server
    let host = host.unwrap_or("127.0.0.1".to_string());
    let addr = format!("{}:{}", host, port);
    let Ok(listener) = tokio::net::TcpListener::bind(addr).await else {
      return Err(Error::from_reason("Failed to bind to address"));
    };
    let server = axum::serve(listener, self.router.lock().unwrap().clone());
    if let Err(e) = server.await {
      tracing::error!("Server error: {}", e);
    }
    Ok(true)
  }

  #[napi]
  pub fn get(
    &mut self,
    path: String,
    handler: ThreadsafeFunction<serde_json::Value, Fatal>,
  ) -> Result<bool> {
    let router = self.router.lock().unwrap().clone();
    self.router = router
      .route(
        &path,
        axum::routing::get(|req| async move { process_request(req, handler).await }),
      )
      .into();
    Ok(true)
  }
}

async fn process_request(
  req: Request<Body>,
  handler: ThreadsafeFunction<serde_json::Value, Fatal>,
) -> Response<Body> {
  let mut url = req.uri().path().to_string();
  if let Some(query) = req.uri().query() {
    url = format!("{}?{}", url, query);
  }
  let request = serde_json::json!({
    "method": req.method().as_str(),
    "url": url,
    "headers": req.headers()
      .iter()
      .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or("")))
      .collect::<serde_json::Value>(),
  });
  let response = serde_json::json!({
    "headersSent": false,
  });
  let call_data = serde_json::json!({
    "request": request,
    "response": response,
  });
  let mut res = Response::builder();
  let js_res = handler.call_async::<Promise<Value>>(call_data);
  let js_data =
    match js_res.await {
      Ok(async_body) => match async_body.await {
        Ok(body) => body,
        Err(err) => return res_error(err),
      },
      Err(err) => return res_error(err),
    };
  res = res.status(StatusCode::OK);
  res.body(Body::from(js_data.to_string())).unwrap()
}

fn res_error(err: Error) -> Response<Body> {
  let err = err.to_string();
  let mut res = Response::builder();
  let error = err
    .split("Error: ")
    .collect::<Vec<&str>>()
    .pop()
    .unwrap_or(&err);
  let Ok(err_json) = serde_json::from_str::<serde_json::Value>(error) else {
    res = res.status(StatusCode::INTERNAL_SERVER_ERROR);
    let err_json = serde_json::json!({
      "status": 500,
      "message": error,
    });
    return res.body(Body::from(err_json.to_string())).unwrap();
  };
  let status = err_json["status"].as_u64().unwrap_or(500) as u16;
  res = res.status(StatusCode::from_u16(status).unwrap());
  res.body(Body::from(err_json.to_string())).unwrap()
}
