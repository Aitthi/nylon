use napi_derive::napi;

#[napi]
pub fn info(log: String) {
  let span = tracing::span!(tracing::Level::INFO, "Js");
  tracing::info!(parent: &span, "{}", log);
}

#[napi]
pub fn debug(log: String) {
  let span = tracing::span!(tracing::Level::INFO, "Js");
  tracing::debug!(parent: &span, "{}", log);
}

#[napi]
pub fn error(log: String) {
  let span = tracing::span!(tracing::Level::INFO, "Js");
  tracing::error!(parent: &span, "{}", log);
}

#[napi]
pub fn warn(log: String) {
  let span = tracing::span!(tracing::Level::INFO, "Js");
  tracing::warn!(parent: &span, "{}", log);
}

#[napi]
pub fn trace(log: String) {
  let span = tracing::span!(tracing::Level::INFO, "Js");
  tracing::trace!(parent: &span, "{}", log);
}