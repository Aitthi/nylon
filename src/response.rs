use std::collections::HashMap;

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Response {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    #[serde(rename = "headersSent")]
    headers_sent: bool,
    is_end: bool,
}

impl Response {
    pub fn into_parts(self) -> (u16, HashMap<String, String>, Vec<u8>, bool, bool) {
        (
            self.status,
            self.headers,
            self.body,
            self.headers_sent,
            self.is_end,
        )
    }
}

#[napi]
impl Response {
    #[napi(constructor)]
    pub fn new(ctx: serde_json::Value) -> Self {
        let binding = serde_json::Map::new();
        let ctx = ctx["response"].as_object().unwrap_or(&binding);
        let mut response =
            Response {
                status: 200,
                headers: HashMap::new(),
                body: Vec::new(),
                headers_sent: false,
                is_end: false,
            };
        let binding = serde_json::Map::new();
        let headers = ctx["headers"].as_object().unwrap_or(&binding);
        for (key, value) in headers {
            response.headers.insert(
                key.to_string(),
                value.as_str().unwrap_or_default().to_string(),
            );
        }
        let Some(buf) = ctx["body"].as_array() else {
            return response;
        };
        // println!("buf: {:?}", buf);
        let mut body = Vec::new();
        for b in buf {
            // safety: unwrap is safe because we know that the body is an array of numbers
            body.push(b.as_u64().unwrap() as u8);
        }
        response.body = body;
        response
    }

    #[napi]
    pub fn end(&mut self) -> serde_json::Value {
        self.headers_sent = true;
        serde_json::json!({
          "status": self.status,
          "headers": self.headers,
          "body": self.body,
          "headersSent": self.headers_sent,
          "is_end": true
        })
    }

    #[napi]
    pub fn next(&mut self) -> serde_json::Value {
        self.headers_sent = true;
        serde_json::json!({
          "status": self.status,
          "headers": self.headers,
          "body": self.body,
          "headersSent": self.headers_sent,
          "is_end": false
        })
    }

    #[napi]
    pub fn status(&mut self, status: u16) -> Result<()> {
        self.status = status;
        Ok(())
    }

    #[napi]
    pub fn header(&mut self, key: String, value: String) -> Result<()> {
        self.headers.insert(key, value);
        Ok(())
    }

    #[napi]
    pub fn json(&mut self, value: serde_json::Value) -> Result<()> {
        let mut headers = self.headers.clone();
        headers.insert("content-type".to_string(), "application/json".to_string());
        self.headers = headers;
        let value = if value.is_null() {
            serde_json::json!({})
        } else {
            value
        };
        self.body = serde_json::to_vec(&value)?;
        Ok(())
    }

    #[napi]
    pub fn send(&mut self, value: serde_json::Value) -> Result<()> {
        let value = if value.is_null() {
            "".to_string()
        } else {
            value.as_str().unwrap_or_default().to_string()
        };
        self.body = value.into_bytes();
        Ok(())
    }

    #[napi]
    pub fn html(&mut self, value: serde_json::Value) -> Result<()> {
        let mut headers = self.headers.clone();
        headers.insert("content-type".to_string(), "text/html".to_string());
        self.headers = headers;
        let _ = self.send(value);
        Ok(())
    }

    #[napi]
    pub fn text(&mut self, value: serde_json::Value) -> Result<()> {
        let mut headers = self.headers.clone();
        headers.insert("content-type".to_string(), "text/plain".to_string());
        self.headers = headers;
        let _ = self.send(value);
        Ok(())
    }

    #[napi]
    pub fn redirect(&mut self, url: String) -> Result<()> {
        self.status = 302;
        let mut headers = self.headers.clone();
        headers.insert("location".to_string(), url);
        self.headers = headers;
        Ok(())
    }
}
