use napi_derive::napi;

#[napi(js_name = "HttpException")]
pub fn http_exception(status: u16, message: String) -> String {
  format!("{{\"status\":{},\"message\":\"{}\"}}", status, message)
}
