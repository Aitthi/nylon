use serde_json::Value;

pub fn body_json(entire_body: &Vec<u8>, request: &mut Value) {
  request["body"] =
    serde_json::json!(serde_json::from_slice::<serde_json::Value>(entire_body.as_slice()).unwrap());
}
