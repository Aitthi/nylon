use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct RequestData {
    headers: HashMap<String, String>,
    body: Vec<u8>,
    method: String,
    url: String,
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
            method: ctx
                .get("method")
                .unwrap_or(&serde_json::Value::Null)
                .as_str()
                .unwrap_or_default()
                .to_string(),
            url: ctx
                .get("url")
                .unwrap_or(&serde_json::Value::Null)
                .as_str()
                .unwrap_or_default()
                .to_string(),
        };
        let binding = serde_json::Map::new();
        let headers = ctx["headers"].as_object().unwrap_or(&binding);
        for (key, value) in headers {
            request.headers.insert(
                key.to_string(),
                value.as_str().unwrap_or_default().to_string(),
            );
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
            None => {
                return Err(Error::from_reason("Content-Type header is required for JSON responses"))
            }
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
            None => {
                return Err(Error::from_reason("Content-Type header is required for text responses"))
            }
        };
        let body = self.request.body.clone();
        let Ok(body) = String::from_utf8(body) else {
            return Err(Error::from_reason("Failed to parse text body"));
        };
        Ok(body)
    }

    #[napi]
    pub fn form(&self, extended: Option<bool>) -> Result<serde_json::Value> {
        let extended = extended.map_or(false, |extended| extended);
        let headers = self.request.headers.clone();
        let content_type = match headers.get("content-type") {
            Some(content_type) => content_type.to_lowercase(),
            None => {
                return Err(Error::from_reason("Content-Type header is required for form responses"))
            }
        };
        if !content_type.contains("application/x-www-form-urlencoded") {
            return Err(
                Error::from_reason("Content-Type header must be application/x-www-form-urlencoded")
            );
        }
        let body = self.request.body.clone();
        let body = String::from_utf8(body).unwrap_or_default();
        if !extended {
            let mut form: HashMap<String, String> = HashMap::new();
            let mut keys: Vec<String> = Vec::new();
            for pair in body.split('&') {
                let mut pair = pair.split('=');
                let key = pair.next().unwrap_or_default();
                let value = pair.next().unwrap_or_default();
                if keys.contains(&key.to_string()) {
                    let mut keys_len = keys.len();
                    if keys_len == 1 {
                        keys_len = 0;
                        let data = match form.get(&key.to_string()) {
                            Some(data) => data.to_string(),
                            None => "".to_string(),
                        };
                        let akey = format!("{}[{}]", key, keys_len);
                        form.insert(akey.to_string(), data.to_string());
                        form.remove(&key.to_string());
                    }
                    let keys_len = keys.len();
                    let key = format!("{}[{}]", key, keys_len);
                    form.insert(key.to_string(), value.to_string());
                    keys.push(key.to_string());
                } else {
                    keys.push(key.to_string());
                    form.insert(key.to_string(), value.to_string());
                }
            }
            return Ok(serde_json::to_value(form).unwrap_or_default());
        }
        // ex: name=John&name=Mike&age=20
        let mut form = HashMap::new();
        for pair in body.split('&') {
            let mut pair = pair.split('=');
            let key = pair.next().unwrap_or_default();
            let value = pair.next().unwrap_or_default();
            let values = form.entry(key.to_string()).or_insert_with(Vec::new);
            values.push(value.to_string());
        }
        Ok(serde_json::to_value(form).unwrap_or_default())
    }

    #[napi]
    pub fn header(&self, key: String) -> Result<String> {
        let headers = self.request.headers.clone();
        // println!("headers: {:#?}", headers);
        let value =
            match headers.get(&key) {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };
        Ok(value)
    }

    #[napi]
    pub fn headers(&self) -> Result<serde_json::Value> {
        let headers = self.request.headers.clone();
        Ok(serde_json::to_value(headers).unwrap_or_default())
    }

    #[napi]
    pub fn method(&self) -> Result<String> {
        let method = self.request.method.clone();
        Ok(method)
    }

    #[napi]
    pub fn url(&self) -> Result<String> {
        let url = self.request.url.clone();
        Ok(url)
    }
}
