use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use std::collections::HashMap;

pub type Handler = ThreadsafeFunction<serde_json::Value, ErrorStrategy::Fatal>;
pub struct RouterResult<'a> {
  pub handler: &'a Handler,
  pub params: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Router {
  routes: matchit::Router<Handler>,
}

impl Router {
  pub fn new() -> Router {
    Router {
      routes: matchit::Router::new(),
    }
  }

  pub fn find<'a>(&'a self, path: &str, method: &str) -> Option<RouterResult> {
    let find_path = format!("/{}{}", method, path);
    let find_path = find_path.as_str();
    // println!("Find: {}",find_path);
    if let Ok(match_route) = self.routes.at(find_path) {
      let params: HashMap<String, String> = match_route
        .params
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
      // return
      Some(RouterResult {
        handler: match_route.value,
        params,
      })
    } else {
      None
    }
  }

  pub fn delegate(&mut self, path: &str, handler: Handler) -> Result<bool, matchit::InsertError> {
    self.routes.insert(format!("{}", path), handler)?;
    Ok(true)
  }
}
