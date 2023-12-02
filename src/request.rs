use bytes::Bytes;
use futures::stream::once;
use multer::{Constraints, Multipart, SizeLimit};
use napi_derive::napi;
use std::{collections::HashMap, convert::Infallible};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct RequestData {
    headers: HashMap<String, String>,
    body: Vec<u8>,
    method: String,
    path: String,
    url: String,
    query: HashMap<String, String>,
    params: HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
struct MultipartOptions {
    pub limit: String, // default 5mb
    pub allowed_fields: Option<Vec<String>>,
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
            path: ctx
                .get("path")
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
            query: HashMap::new(),
            params: HashMap::new(),
        };
        let binding = serde_json::Map::new();
        let headers = ctx["headers"].as_object().unwrap_or(&binding);
        for (key, value) in headers {
            request.headers.insert(
                key.to_string(),
                value.as_str().unwrap_or_default().to_string(),
            );
        }

        let binding = serde_json::Map::new();
        let query = ctx["query"].as_object().unwrap_or(&binding);
        for (key, value) in query {
            request.query.insert(
                key.to_string(),
                value.as_str().unwrap_or_default().to_string(),
            );
        }

        let binding = serde_json::Map::new();
        let params = ctx["params"].as_object().unwrap_or(&binding);
        for (key, value) in params {
            request.params.insert(
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
    pub fn json(&self) -> napi::Result<serde_json::Value> {
        let headers = self.request.headers.clone();
        let content_type = match headers.get("content-type") {
            Some(content_type) => content_type.to_lowercase(),
            None => {
                return Err(
                    napi::Error::from_reason("Content-Type header is required for JSON responses")
                )
            }
        };
        if !content_type.contains("application/json") {
            return Err(napi::Error::from_reason("Content-Type header must be application/json"));
        }
        let Ok(body) = serde_json::from_slice(&self.request.body) else {
            return Err(napi::Error::from_reason("Failed to parse JSON body"));
        };
        Ok(body)
    }

    #[napi]
    pub fn text(&self) -> napi::Result<String> {
        let headers = self.request.headers.clone();
        match headers.get("content-type") {
            Some(content_type) => content_type.to_lowercase(),
            None => {
                return Err(
                    napi::Error::from_reason("Content-Type header is required for text responses")
                )
            }
        };
        let body = self.request.body.clone();
        let Ok(body) = String::from_utf8(body) else {
            return Err(napi::Error::from_reason("Failed to parse text body"));
        };
        Ok(body)
    }

    #[napi]
    pub fn form(&self, extended: Option<bool>) -> napi::Result<serde_json::Value> {
        let extended = extended.map_or(false, |extended| extended);
        let headers = self.request.headers.clone();
        let content_type = match headers.get("content-type") {
            Some(content_type) => content_type.to_lowercase(),
            None => {
                return Err(
                    napi::Error::from_reason("Content-Type header is required for form responses")
                )
            }
        };
        if !content_type.contains("application/x-www-form-urlencoded") {
            return Err(napi::Error::from_reason(
                "Content-Type header must be application/x-www-form-urlencoded",
            ));
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

    #[napi(ts_args_type = "op: { limit: string, allowed_fields: string[] }")]
    pub async fn multipart(&self, op: serde_json::Value) -> napi::Result<serde_json::Value> {
        let headers = self.request.headers.clone();
        let content_type =
            match headers.get("content-type") {
                Some(content_type) => content_type.to_lowercase(),
                None => {
                    return Err(napi::Error::from_reason(
                        "Content-Type header is required for multipart responses",
                    ))
                }
            };
        if !content_type.contains("multipart/form-data") {
            return Err(napi::Error::from_reason("Content-Type header must be multipart/form-data"));
        }
        let data = self.request.body.clone();
        let boundary =
            match self.request.headers.get("content-type") {
                Some(content_type) => content_type.to_lowercase(),
                None => {
                    return Err(napi::Error::from_reason(
                        "Content-Type header is required for multipart responses",
                    ))
                }
            };
        let boundary = boundary
            .split("boundary=")
            .collect::<Vec<&str>>()
            .pop()
            .unwrap_or_default();
        let mut constraints = Constraints::new();
        let op: MultipartOptions = serde_json::from_value(op).unwrap_or_default();
        let limit = op.limit; // only support kb, mb
        let mut size_limit = SizeLimit::new();
        if limit.ends_with("kb") {
            let limit = limit.replace("kb", "");
            let limit = limit.parse::<u64>().unwrap_or_default();
            size_limit = size_limit.whole_stream(limit * 1024);
        } else if limit.ends_with("mb") {
            let limit = limit.replace("mb", "");
            let limit = limit.parse::<u64>().unwrap_or_default();
            size_limit = size_limit.whole_stream(limit * 1024 * 1024);
        } else {
            // default 5mb
            size_limit = size_limit.whole_stream(5 * 1024 * 1024);
        }
        constraints = constraints.size_limit(size_limit);

        if let Some(allowed_fields) = op.allowed_fields {
            constraints = constraints.allowed_fields(allowed_fields);
        }

        let data = Bytes::from(data);
        let stream = once(async move { Result::<Bytes, Infallible>::Ok(data) });
        let mut multipart = Multipart::with_constraints(stream, boundary, constraints);
        let mut form: HashMap<String, serde_json::Value> = HashMap::new();
        while let Some(mut field) = match multipart.next_field().await {
            Ok(field) => field,
            Err(e) => {
                let e = format!("{:?}", e);
                return Err(napi::Error::from_reason(e.as_str()));
            }
        } {
            let name = field.name().map(|name| name.to_string());
            let file_name = field.file_name();
            let file_name = file_name.map(|file_name| file_name.to_string());
            let is_file = file_name.is_some();
            let content_type = field.content_type();
            let content_type = content_type.map(|content_type| content_type.to_string());
            if let Some(name) = name {
                // Process the field data chunks e.g. store them in a file.
                while let Some(chunk) = match field.chunk().await {
                    Ok(chunk) => chunk,
                    _ => Some(Bytes::new()),
                } {
                    if !is_file {
                        form.insert(
                            name.to_string(),
                            String::from_utf8(chunk.to_vec()).unwrap_or_default().into(),
                        );
                    } else {
                        let file = chunk.to_vec();
                        let file = serde_json::json!({
                            "name": file_name.clone().unwrap_or_default(),
                            "type": content_type.clone().unwrap_or_default(),
                            "data": file,
                            "size": file.len(),
                        });
                        form.insert(name.to_string(), file);
                    }
                }
            }
        }
        Ok(serde_json::to_value(form).unwrap_or_default())
    }

    #[napi]
    pub fn header(&self, key: String) -> napi::Result<String> {
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
    pub fn headers(&self) -> napi::Result<serde_json::Value> {
        let headers = self.request.headers.clone();
        Ok(serde_json::to_value(headers).unwrap_or_default())
    }

    #[napi]
    pub fn method(&self) -> napi::Result<String> {
        let method = self.request.method.clone();
        Ok(method)
    }

    #[napi]
    pub fn path(&self) -> napi::Result<String> {
        let url = self.request.path.clone();
        Ok(url)
    }

    #[napi]
    pub fn url(&self) -> napi::Result<String> {
        let url = self.request.url.clone();
        Ok(url)
    }

    #[napi]
    pub fn query(&self, key: String) -> napi::Result<String> {
        let query = self.request.query.clone();
        let value = match query.get(&key) {
            Some(value) => value.to_string(),
            None => "".to_string(),
        };
        Ok(value)
    }

    #[napi]
    pub fn queries(&self) -> napi::Result<serde_json::Value> {
        let query = self.request.query.clone();
        Ok(serde_json::to_value(query).unwrap_or_default())
    }

    #[napi]
    pub fn param(&self, key: String) -> napi::Result<String> {
        let params = self.request.params.clone();
        let value =
            match params.get(&key) {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };
        Ok(value)
    }

    #[napi]
    pub fn params(&self) -> napi::Result<serde_json::Value> {
        let params = self.request.params.clone();
        Ok(serde_json::to_value(params).unwrap_or_default())
    }
}
