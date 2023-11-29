use napi::{bindgen_prelude::*, JsArrayBuffer};
use napi_derive::napi;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct RequestData {
  headers: HashMap<String, String>,
  body: Vec<u8>,
}

#[napi]
pub struct Request {
  request: RequestData,
}

#[napi]
impl Request {
  #[napi(constructor)]
  pub fn new(ctx: serde_json::Value) -> Self {
    let binding = serde_json::Map::new();
    let ctx = ctx["request"].as_object().unwrap_or(&binding);
    let mut request = RequestData {
      headers: HashMap::new(),
      body: Vec::new(),
    };
    let binding = serde_json::Map::new();
    let headers = ctx["headers"].as_object().unwrap_or(&binding);
    for (key, value) in headers {
      request.headers.insert(key.to_string(), value.to_string());
    }
    let Some(buf) = ctx["body"].as_array() else {
      return Self { request };
    };
    // println!("buf: {:?}", buf);
    let mut body = Vec::new();
    for b in buf {
      // safety: unwrap is safe because we know that the body is an array of numbers
      body.push(b.as_u64().unwrap() as u8);
    }
    request.body = body;
    Request { request }
  }

  #[napi]
  pub fn json(&self) -> Result<serde_json::Value> {
    let headers = self.request.headers.clone();
    let content_type = match headers.get("content-type") {
      Some(content_type) => content_type.to_lowercase(),
      None => return Err(Error::from_reason("Content-Type header is required for JSON responses")),
    };
    if !content_type.contains("application/json") {
      return Err(Error::from_reason("Content-Type header must be application/json"));
    }
    let Ok(body) = serde_json::from_slice(&self.request.body) else {
      return Err(Error::from_reason("Failed to parse JSON body"));
    };
    Ok(body)
  }

  #[napi]
  pub fn text(&self) -> Result<String> {
    let headers = self.request.headers.clone();
    match headers.get("content-type") {
      Some(content_type) => content_type.to_lowercase(),
      None => return Err(Error::from_reason("Content-Type header is required for text responses")),
    };
    let body = self.request.body.clone();
    let Ok(body) = String::from_utf8(body) else {
      return Err(Error::from_reason("Failed to parse text body"));
    };
    Ok(body)
  }

  #[napi]
  pub fn form(&self) -> Result<HashMap<String, String>> {
    let headers = self.request.headers.clone();
    let content_type = match headers.get("content-type") {
      Some(content_type) => content_type.to_lowercase(),
      None => return Err(Error::from_reason("Content-Type header is required for form responses")),
    };
    if !content_type.contains("application/x-www-form-urlencoded") {
      return Err(
        Error::from_reason("Content-Type header must be application/x-www-form-urlencoded")
      );
    }
    let body = self.request.body.clone();
    let Ok(body) = String::from_utf8(body) else {
      return Err(Error::from_reason("Failed to parse form body"));
    };
    let mut form = HashMap::new();
    for pair in body.split('&') {
      let mut pair = pair.split('=');
      let key = pair.next().unwrap_or_default();
      let value = pair.next().unwrap_or_default();
      form.insert(key.to_string(), value.to_string());
    }
    Ok(form)
  }
}
