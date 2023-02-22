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
use serde::{Deserialize, Serialize};
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
    tokio::task::spawn(async { Ok(true) }).await.unwrap()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct JsResponse {
    data: String,
    headres: HashMap<String, String>,
    res_code: u16,
    is_json: bool,
    json: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsRequestBody {
    pub buffer: Vec<u8>,
    pub json: serde_json::Value,
    pub data: String
}
impl JsRequestBody {
    pub fn new() -> Self {
        JsRequestBody {
            buffer: vec![],
            json: serde_json::Value::Null,
            data: String::default()
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct JsRequest {
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub query: serde_json::Value,
    pub method: String,
    pub path: String,
    pub body: JsRequestBody,
}

impl JsRequest {
    pub fn new() -> Self {
        JsRequest {
            headers: HashMap::default(),
            params: HashMap::default(),
            query:  serde_json::Value::Null,
            method: String::default(),
            path: String::default(),
            body: JsRequestBody::new()
        }
    }
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
    // to js
    let mut js_request = JsRequest::new();
    js_request.method = method.to_string();
    js_request.path = path.path().to_string();
    for (key, val) in headers.iter() {
        js_request.headers.insert(key.to_string(), val.to_str().unwrap().to_string());
    }
    if let Some(raw_query) = raw_query {
        let full_url = format!("htttp://localhost?{}", raw_query);
        let url_parse = url::Url::parse(full_url.as_str()).unwrap();
        js_request.query = url_parse.query_pairs().into_owned().collect();
    }
    // JsRequestBody
    let mut js_request_body = JsRequestBody::new();
    js_request_body.buffer = entire_body.clone();
    if !js_request_body.buffer.is_empty() {
        if let Some(content_type) = headers.get("content-type") {
            if content_type.to_str().unwrap() == "application/json" {
                js_request_body.json = serde_json::from_slice::<serde_json::Value>(entire_body.as_slice()).unwrap();
            } else if vec![
                "text/html",
                "text/plain",
                "application/javascript",
                "application/xml"
            ].contains(&content_type.to_str().unwrap()) {
                js_request_body.data = String::from_utf8_lossy(entire_body.as_slice()).to_string();
            }
        }
    }
    js_request.body = js_request_body;
    let mut builder = hyper::Response::builder();
    if let Some(route) = routes.find(path.path(), method.as_str()) {
        let handler = route.handler;
        js_request.params = route.params;
        // https://github.com/napi-rs/napi-rs/issues/1469
        // 'Send Promise resolved value error'
        match handler
            .call_async::<Promise<serde_json::Value>>(Ok(serde_json::to_value(js_request).unwrap()))
            .await
        {
            Ok(js_promise) => match js_promise.await {
                Ok(js_data) => {
                    let js_res: JsResponse = serde_json::from_value(js_data).unwrap();
                    builder = builder.status(js_res.res_code);
                    let buf_array: Vec<u8>;
                    if js_res.is_json {
                        buf_array = serde_json::to_vec(&js_res.json).unwrap()
                    } else {
                        buf_array = js_res.data.as_bytes().to_vec();
                    }
                    for header in js_res.headres {
                        builder = builder.header(header.0, header.1);
                    }
                    let res = builder.body(hyper::Body::from(buf_array)).unwrap();
                    return Ok(res);
                }
                Err(e) => {
                    builder = builder.status(500);
                    let res = builder.body(hyper::Body::from(format!("{:?}", e))).unwrap();
                    return Ok(res);
                }
            },
            Err(e) => {
                builder = builder.status(500);
                let res = builder.body(hyper::Body::from(format!("{:?}", e))).unwrap();
                return Ok(res);
            }
        }
    } else {
        builder = builder.status(404);
        let res = builder.body(hyper::Body::from("Not found")).unwrap();
        Ok(res)
    }
}
