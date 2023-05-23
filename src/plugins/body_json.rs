use hyper::{http::HeaderValue, HeaderMap};
use serde_json::Value;

pub fn body_json(
  entire_body: &Vec<u8>,
  headers: HeaderMap<HeaderValue>,
  allow_content_type: Vec<String>,
  request: &mut Value,
) {
  if let Some(content_type) = headers.get("content-type") {
    if allow_content_type.contains(&content_type.to_str().unwrap().to_string()) {
      request["body"] = serde_json::json!(serde_json::from_slice::<serde_json::Value>(
        entire_body.as_slice()
      )
      .unwrap());
    }
  }
}
