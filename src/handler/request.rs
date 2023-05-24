use super::http_body::parse_body;
use super::url_parse::url_parse;
use crate::router;
use napi::bindgen_prelude::Promise;
use napi::bindgen_prelude::*;
use std::collections::HashMap;

pub async fn request(
  req: hyper::Request<hyper::Body>,
) -> core::result::Result<hyper::Response<hyper::Body>, Error> {
  let (parts, body) = req.into_parts();
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
      "headers": {
        "content-type": "application/json"
      },
      "body": serde_json::Value::Null,
      "raw_body": Vec::<u8>::new(),
  });

  // parse query
  url_parse(raw_query, &mut request);
  // parse body
  parse_body(body, headers, &mut request).await;

  let mut builder = hyper::Response::builder();
  if let Some(route) = routes.find(path.path(), method.as_str()) {
    request["params"] = serde_json::json!(route.params);
    // call handler
    if let Ok(res_data) = route
      .handler
      .call_async::<Promise<serde_json::Value>>(serde_json::json!(request))
      .await
    {
      if let Ok(res) = res_data.await {
        response = res;
      }
    }
    builder = builder.status(response["status"].as_u64().unwrap_or(200) as u16);
    for (k, v) in response["headers"].as_object().unwrap_or(
      serde_json::json!({
          "content-type": "application/json"
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
