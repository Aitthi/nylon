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
