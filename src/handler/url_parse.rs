use std::collections::HashMap;

use serde_json::Value;

pub fn url_parse(raw_query: Option<&str>, request: &mut Value) {
  if let Some(raw_query) = raw_query {
    let full_url = format!("htttp://localhost?{}", raw_query);
    let url_parse = url::Url::parse(full_url.as_str()).unwrap();
    request["query"] = serde_json::json!(url_parse
      .query_pairs()
      .into_owned()
      .collect::<HashMap<String, String>>());
  }
}
