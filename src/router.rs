use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use std::collections::HashMap;

pub type Handler = ThreadsafeFunction<serde_json::Value, ErrorStrategy::Fatal>;
pub struct RouterResult<'a> {
    pub handlers: &'a Vec<Handler>,
    pub params: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Router {
    routes: matchit::Router<Vec<Handler>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: matchit::Router::new(),
        }
    }

    pub fn find<'a>(&'a self, path: &str, method: &str) -> Option<RouterResult> {
        let find_path = format!("[{}] {}", method, path);
        let find_path = find_path.as_str();
        // println!("Find: {}",find_path);
        match self.routes.at(find_path) {
            Ok(match_) => {
                // println!("{:#?}", match_.params);
                let params: HashMap<String, String> = match_
                    .params
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();
                Some(RouterResult {
                    handlers: match_.value,
                    params,
                })
            }
            Err(_) => None,
        }
    }

    pub fn delegate(
        &mut self,
        path: &str,
        handlers: Vec<Handler>,
    ) -> Result<bool, matchit::InsertError> {
        self.routes.insert(format!("{}", path), handlers)?;
        Ok(true)
    }
}
