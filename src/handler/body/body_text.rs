use serde_json::Value;

pub fn body_text(entire_body: &Vec<u8>, request: &mut Value) {
  request["body"] =
    serde_json::Value::String(String::from_utf8_lossy(entire_body.as_slice()).to_string());
}
