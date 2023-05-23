use napi_derive::napi;

#[napi]
pub fn set_env(key: String, value: String) {
  std::env::set_var(key, value);
}