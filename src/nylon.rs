use crate::response::Response as NylonResponse;
use axum::{
    body::Body,
    extract::{Path, Query, Request},
    http::{Response, StatusCode},
    routing, Router,
};
use futures_util::stream::TryStreamExt;
use napi::{
    bindgen_prelude::*,
    threadsafe_function::{ErrorStrategy::Fatal, ThreadsafeFunction},
};
use napi_derive::napi;
use serde_json::Value;
use std::{collections::HashMap, sync::Mutex};

#[napi]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Trace,
    Any,
}

impl Method {
    pub fn from_str_method(method: &str) -> Self {
        match method {
            "get" => Method::Get,
            "post" => Method::Post,
            "put" => Method::Put,
            "delete" => Method::Delete,
            "patch" => Method::Patch,
            "head" => Method::Head,
            "options" => Method::Options,
            "trace" => Method::Trace,
            _ => Method::Get,
        }
    }
}

#[derive(Clone, Default)]
#[napi]
pub struct Nylon {
    router: HashMap<String, HashMap<String, Vec<ThreadsafeFunction<serde_json::Value, Fatal>>>>,
}

#[napi]
impl Nylon {
    #[napi(constructor)]
    pub fn new() -> Self {
        // Setup tracing
        tracing::info!("Starting Nylon application...");
        Nylon {
            router: HashMap::new(),
        }
    }

    #[napi]
    pub async fn listen(
        &self,
        port: u16,
        host: Option<String>,
        callback: ThreadsafeFunction<()>,
    ) -> Result<bool> {
        // Start the server
        let host = host.unwrap_or("127.0.0.1".to_string());
        let addr = format!("{}:{}", host, port);
        let Ok(listener) = tokio::net::TcpListener::bind(addr).await else {
            return Err(Error::from_reason("Failed to bind to address"));
        };

        let mut svc_router = Mutex::new(Router::new());
        for (path, method_handler) in &self.router {
            for (method, handler) in method_handler {
                let handler = handler.clone();
                let router = svc_router.lock().unwrap().clone();
                let add_router_time = std::time::Instant::now();
                match Method::from_str_method(method) {
                    Method::Get => {
                        svc_router = router
                            .route(
                                path,
                                routing::get(
                                    |params: Path<HashMap<String, String>>,
                                     query: Query<HashMap<String, String>>,
                                     req: Request<Body>| async move {
                                        process_request(req, params, query, handler).await
                                    },
                                ),
                            )
                            .into();
                    }
                    Method::Post => {
                        svc_router = router
                            .route(
                                path,
                                routing::post(
                                    |params: Path<HashMap<String, String>>,
                                     query: Query<HashMap<String, String>>,
                                     req: Request<Body>| async move {
                                        process_request(req, params, query, handler).await
                                    },
                                ),
                            )
                            .into();
                    }
                    Method::Put => {
                        svc_router = router
                            .route(
                                path,
                                routing::put(
                                    |params: Path<HashMap<String, String>>,
                                     query: Query<HashMap<String, String>>,
                                     req: Request<Body>| async move {
                                        process_request(req, params, query, handler).await
                                    },
                                ),
                            )
                            .into();
                    }
                    Method::Delete => {
                        svc_router = router
                            .route(
                                path,
                                routing::delete(
                                    |params: Path<HashMap<String, String>>,
                                     query: Query<HashMap<String, String>>,
                                     req: Request<Body>| async move {
                                        process_request(req, params, query, handler).await
                                    },
                                ),
                            )
                            .into();
                    }
                    Method::Patch => {
                        svc_router = router
                            .route(
                                path,
                                routing::patch(
                                    |params: Path<HashMap<String, String>>,
                                     query: Query<HashMap<String, String>>,
                                     req: Request<Body>| async move {
                                        process_request(req, params, query, handler).await
                                    },
                                ),
                            )
                            .into();
                    }
                    Method::Head => {
                        svc_router = router
                            .route(
                                path,
                                routing::head(
                                    |params: Path<HashMap<String, String>>,
                                     query: Query<HashMap<String, String>>,
                                     req: Request<Body>| async move {
                                        process_request(req, params, query, handler).await
                                    },
                                ),
                            )
                            .into();
                    }
                    Method::Options => {
                        svc_router = router
                            .route(
                                path,
                                routing::options(
                                    |params: Path<HashMap<String, String>>,
                                     query: Query<HashMap<String, String>>,
                                     req: Request<Body>| async move {
                                        process_request(req, params, query, handler).await
                                    },
                                ),
                            )
                            .into();
                    }
                    Method::Trace => {
                        svc_router = router
                            .route(
                                path,
                                routing::trace(
                                    |params: Path<HashMap<String, String>>,
                                     query: Query<HashMap<String, String>>,
                                     req: Request<Body>| async move {
                                        process_request(req, params, query, handler).await
                                    },
                                ),
                            )
                            .into();
                    }
                    Method::Any => {
                        svc_router = router
                            .route(
                                path,
                                routing::any(
                                    |params: Path<HashMap<String, String>>,
                                     query: Query<HashMap<String, String>>,
                                     req: Request<Body>| async move {
                                        process_request(req, params, query, handler).await
                                    },
                                ),
                            )
                            .into();
                    }
                };
                let add_router_time = add_router_time.elapsed().as_micros();
                let scope = tracing::span!(tracing::Level::INFO, "router",);
                scope.in_scope(|| {
                    tracing::info!("{} {} +{}us", method, path, add_router_time);
                });
            }
        }

        let server = axum::serve(listener, svc_router.lock().unwrap().clone());
        callback.call(
            Ok(()),
            napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
        );
        if let Err(e) = server.await {
            tracing::error!("Server error: {}", e);
        }
        Ok(true)
    }

    pub fn route(
        &mut self,
        path: &str,
        method: Method,
        handler: Vec<ThreadsafeFunction<serde_json::Value, Fatal>>,
    ) {
        let method = match method {
            Method::Get => "get",
            Method::Post => "post",
            Method::Put => "put",
            Method::Delete => "delete",
            Method::Patch => "patch",
            Method::Head => "head",
            Method::Options => "options",
            Method::Trace => "trace",
            Method::Any => "any",
        };
        let mut router = self.router.clone();
        if let Some(method_handler) = router.get_mut(path) {
            method_handler.insert(method.to_string(), handler);
        } else {
            let mut method_handler = HashMap::new();
            method_handler.insert(method.to_string(), handler);
            router.insert(path.to_string(), method_handler);
        }
        self.router = router;
    }

    #[napi]
    pub fn get(
        &mut self,
        path: String,
        handler: Vec<ThreadsafeFunction<serde_json::Value, Fatal>>,
    ) {
        self.route(path.as_str(), Method::Get, handler);
    }

    #[napi]
    pub fn post(
        &mut self,
        path: String,
        handler: Vec<ThreadsafeFunction<serde_json::Value, Fatal>>,
    ) {
        self.route(path.as_str(), Method::Post, handler);
    }

    #[napi]
    pub fn put(
        &mut self,
        path: String,
        handler: Vec<ThreadsafeFunction<serde_json::Value, Fatal>>,
    ) {
        self.route(path.as_str(), Method::Put, handler);
    }

    #[napi]
    pub fn delete(
        &mut self,
        path: String,
        handler: Vec<ThreadsafeFunction<serde_json::Value, Fatal>>,
    ) {
        self.route(path.as_str(), Method::Delete, handler);
    }

    #[napi]
    pub fn patch(
        &mut self,
        path: String,
        handler: Vec<ThreadsafeFunction<serde_json::Value, Fatal>>,
    ) {
        self.route(path.as_str(), Method::Patch, handler);
    }

    #[napi]
    pub fn head(
        &mut self,
        path: String,
        handler: Vec<ThreadsafeFunction<serde_json::Value, Fatal>>,
    ) {
        self.route(path.as_str(), Method::Head, handler);
    }

    #[napi]
    pub fn options(
        &mut self,
        path: String,
        handler: Vec<ThreadsafeFunction<serde_json::Value, Fatal>>,
    ) {
        self.route(path.as_str(), Method::Options, handler);
    }

    #[napi]
    pub fn trace(
        &mut self,
        path: String,
        handler: Vec<ThreadsafeFunction<serde_json::Value, Fatal>>,
    ) {
        self.route(path.as_str(), Method::Trace, handler);
    }
}

async fn process_request(
    req: Request<Body>,
    params: Path<HashMap<String, String>>,
    query: Query<HashMap<String, String>>,
    handlers: Vec<ThreadsafeFunction<serde_json::Value, Fatal>>,
) -> Response<Body> {
    let mut handlers = handlers;
    let path = req.uri().path().to_string();
    let mut url = path.clone();
    if let Some(query) = req.uri().query() {
        url = format!("{}?{}", path, query);
    }
    let (parts, body) = req.into_parts();
    let version: String = format!("{:?}", parts.version);
    let entire_body = body
        .into_data_stream()
        .try_fold(Vec::new(), |mut data, chunk| async move {
            data.extend_from_slice(&chunk);
            Ok(data)
        })
        .await
        .unwrap_or_default();

    let method = parts.method;
    let headers = parts.headers;
    let request = serde_json::json!({
        "method": method.as_str(),
        "path": path,
        "url": url,
        "version": version,
        "headers": headers
          .iter()
          .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or("")))
          .collect::<serde_json::Value>(),
        "body": entire_body,
        "query": query.0,
        "params": params.0,
    });
    // println!("request: {:#?}", request);
    let response = serde_json::json!({
      "headersSent": false,
      "status": 200,
      "headers": HashMap::<String, String>::new(),
      "body": Vec::<u8>::new(),
    });
    let mut call_data = serde_json::json!({
      "request": request,
      "response": response,
    });
    let mut res = Response::builder();
    let mut res_status = 200;
    let mut res_headers = HashMap::new();
    let mut res_body = Vec::new();
    while !handlers.is_empty() {
        let handler = handlers.remove(0);
        let js_res = handler.call_async::<Promise<Value>>(call_data.clone());
        let js_data: NylonResponse = match js_res.await {
            Ok(async_body) => match async_body.await {
                Ok(body) => match serde_json::from_value(body) {
                    Ok(body) => body,
                    Err(err) => return res_error(err.into(), 400),
                },
                Err(err) => return res_error(err, 400),
            },
            Err(err) => return res_error(err, 500),
        };
        let (status, headers, body, _, is_end) = js_data.into_parts();
        call_data["response"] = serde_json::json!({
          "headersSent": true,
          "status": status,
          "headers": headers,
          "body": body,
        });
        res_status = status;
        res_headers = headers;
        res_body = body;
        if is_end {
            break;
        }
    }
    res = res.status(StatusCode::from_u16(res_status).unwrap());
    for (key, value) in res_headers {
        res = res.header(key, value);
    }
    res.body(Body::from(res_body)).unwrap()
}

fn res_error(err: Error, status: u16) -> Response<Body> {
    let err = err.to_string();
    let mut res = Response::builder();
    let error = err
        .split("Error: ")
        .collect::<Vec<&str>>()
        .pop()
        .unwrap_or(&err);
    let Ok(err_json) = serde_json::from_str::<serde_json::Value>(error) else {
        res = res.status(StatusCode::from_u16(status).unwrap());
        let err_json = serde_json::json!({
          "status": status,
          "message": error,
        });
        return res.body(Body::from(err_json.to_string())).unwrap();
    };
    let status = err_json["status"].as_u64().unwrap_or(500) as u16;
    res = res.status(StatusCode::from_u16(status).unwrap());
    res.body(Body::from(err_json.to_string())).unwrap()
}
