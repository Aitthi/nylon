#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
// mod
pub mod logger;
pub mod router;

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
    tokio::task::spawn(async { Ok(true) }).await.unwrap()
}
async fn process(
    req: hyper::Request<hyper::Body>,
) -> core::result::Result<hyper::Response<hyper::Body>, Error> {
    let (parts, body) = req.into_parts();
    let entire_body = body
        .try_fold(Vec::new(), |mut data, chunk| async move {
            data.extend_from_slice(&chunk);
            Ok(data)
        })
        .await
        .unwrap();
    // hyper
    let routes = parts.extensions.get::<router::Router>().unwrap();
    let path = parts.uri.clone();
    let raw_query = path.query();
    let method = parts.method.clone();
    let headers = parts.headers;
    let mut request = serde_json::json!({
        "path": path.path(),
        "method": method.as_str(),
        "headers": headers.iter().map(|(k, v)| (k.as_str(), v.to_str().unwrap())).collect::<HashMap<&str, &str>>(),
        "params": {},
        "query": {},
        "body": serde_json::Value::Null,
    });
    let mut response = serde_json::json!({
        "is_end": false,
        "status": 200,
        "headers": {},
        "body": serde_json::Value::Null,
    });
    if let Some(raw_query) = raw_query {
        let full_url = format!("htttp://localhost?{}", raw_query);
        let url_parse = url::Url::parse(full_url.as_str()).unwrap();
        request["query"] = serde_json::json!(url_parse
            .query_pairs()
            .into_owned()
            .collect::<HashMap<String, String>>());
    }
    if !entire_body.is_empty() {
        if let Some(content_type) = headers.get("content-type") {
            if content_type.to_str().unwrap() == "application/json" {
                request["body"] = serde_json::json!(serde_json::from_slice::<serde_json::Value>(
                    entire_body.as_slice()
                )
                .unwrap());
            } else if vec![
                "text/html",
                "text/plain",
                "application/javascript",
                "application/xml",
            ]
            .contains(&content_type.to_str().unwrap())
            {
                request["body"] = serde_json::Value::String(
                    String::from_utf8_lossy(entire_body.as_slice()).to_string(),
                );
            } else {
                request["body"] = serde_json::Value::Array(
                    entire_body
                        .iter()
                        .map(|x| serde_json::Value::Number(serde_json::Number::from(*x)))
                        .collect::<Vec<serde_json::Value>>(),
                );
            }
        } else {
            request["body"] = serde_json::Value::Array(
                entire_body
                    .iter()
                    .map(|x| serde_json::Value::Number(serde_json::Number::from(*x)))
                    .collect::<Vec<serde_json::Value>>(),
            );
        }
    }

    let mut builder = hyper::Response::builder();
    if let Some(route) = routes.find(path.path(), method.as_str()) {
        let handlers = route.handlers;
        request["params"] = serde_json::json!(route.params);
        // call handler
        for handler in handlers.iter() {
            if let Ok(res_data) = handler
                .call_async::<Promise<serde_json::Value>>(serde_json::json!({
                        "request": request,
                        "response": response
                }))
                .await
            {
                if let Ok(res) = res_data.await {
                    response = res;
                    if response["is_end"].as_bool().unwrap_or(false) {
                        break;
                    }
                }
            }
        }
        builder = builder.status(response["status"].as_u64().unwrap_or(200) as u16);
        for (k, v) in response["headers"].as_object().unwrap_or(
            serde_json::json!({
                "content-type": "text/plain"
            })
            .as_object()
            .unwrap(),
        ) {
            builder = builder.header(k, v.as_str().unwrap());
        }
        let response_data = response["body"]
            .as_str()
            .unwrap_or(response["body"].to_string().as_str())
            .as_bytes()
            .to_vec();
        let res = builder.body(hyper::Body::from(response_data)).unwrap();
        Ok(res)
    } else {
        builder = builder.status(404);
        let res = builder.body(hyper::Body::from("Not Found")).unwrap();
        Ok(res)
    }
}
