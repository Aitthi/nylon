use futures_util::stream::TryStreamExt;
use hyper::{http::HeaderValue, HeaderMap};
use serde_json::Value;

use crate::plugins::body_json;

pub async fn parse_body(body: hyper::Body, headers: HeaderMap<HeaderValue>, request: &mut Value) {
  let entire_body = body
    .try_fold(Vec::new(), |mut data, chunk| async move {
      data.extend_from_slice(&chunk);
      Ok(data)
    })
    .await
    .unwrap();
  request["raw_body"] = entire_body.clone().into();
  if !entire_body.is_empty() {
    body_json(
      &entire_body,
      headers,
      vec!["application/json".to_string()],
      request,
    );
    // if let Some(content_type) = headers.get("content-type") {
    //   if content_type.to_str().unwrap() == "application/json" {
    //     request["body"] = serde_json::json!(serde_json::from_slice::<serde_json::Value>(
    //       entire_body.as_slice()
    //     )
    //     .unwrap());
    //   } else if vec![
    //     "text/html",
    //     "text/plain",
    //     "application/javascript",
    //     "application/xml",
    //   ]
    //   .contains(&content_type.to_str().unwrap())
    //   {
    //     request["body"] =
    //       serde_json::Value::String(String::from_utf8_lossy(entire_body.as_slice()).to_string());
    //   }
    // }
  }
}
