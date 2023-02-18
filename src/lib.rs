#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
// mod
pub mod router;
pub mod logger;
// use
use futures_util::stream::TryStreamExt;
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
    routes: HashMap<String, router::Handler>,
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
    let service = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(registers))
        .service_fn(process);
    let server = Server::bind(&addr).serve(Shared::new(service));
    callback.call(Ok(()), ThreadsafeFunctionCallMode::NonBlocking);
    tracing::info!(
        "Nylon application successfully started +{}ms",
        app_start_time.elapsed().as_millis()
    );
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    tokio::task::spawn(async{Ok(true)}).await.unwrap()
}

async fn process(
    req: hyper::Request<hyper::Body>,
) -> core::result::Result<hyper::Response<hyper::Body>, Error> {
    let (parts, body) = req.into_parts();
    let _entire_body = body
        .try_fold(Vec::new(), |mut data, chunk| async move {
            data.extend_from_slice(&chunk);
            Ok(data)
        })
        .await
        .unwrap();
    let routes = parts.extensions.get::<router::Router>().unwrap();
    let path = parts.uri.clone();
    let method = parts.method.clone();
    let mut builder = hyper::Response::builder();
    if let Some(route) = routes.find(path.path(), method.as_str()) {
        builder = builder.status(200);
        let handlers = route.handlers;
        let msg: serde_json::Value = handlers.get(0).unwrap().call_async(Ok(())).await?;
        let res = builder.body(hyper::Body::from(msg.to_string())).unwrap();
        Ok(res)
    } else {
        builder = builder.status(404);
        let res = builder.body(hyper::Body::from("Not found")).unwrap();
        Ok(res)
    }
}