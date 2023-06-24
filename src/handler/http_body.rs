use futures_util::stream::TryStreamExt;
use hyper::{http::HeaderValue, HeaderMap};
use serde_json::Value;

use super::body::{body_json, body_text};

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
    if let Some(content_type) = headers.get("content-type") {
      if vec!["application/json"].contains(&content_type.to_str().unwrap()) {
        body_json(&entire_body, request);
      } else if vec![
        "text/html",
        "text/plain",
        "application/javascript",
        "application/xml",
      ]
      .contains(&content_type.to_str().unwrap())
      {
        body_text(&entire_body, request);
      }
    }
  }
}
