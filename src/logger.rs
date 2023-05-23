use napi_derive::napi;

const NAME_JS: &str = "NylonJS";

#[napi]
pub struct Logger {}

#[napi]
impl Logger {
  #[napi]
  pub fn init() -> Self {
    Logger {}
  }

  #[napi]
  pub fn info(&self, log: String, span_name: String) {
    let span = tracing::span!(tracing::Level::INFO, NAME_JS, "{}", span_name.as_str());
    span.in_scope(|| {
      tracing::info!("{}", log);
    });
    drop(span);
  }

  #[napi]
  pub fn debug(&self, log: String, span_name: String) {
    let span = tracing::span!(tracing::Level::DEBUG, NAME_JS, "{}", span_name.as_str());
    span.in_scope(|| {
      tracing::debug!("{}", log);
    });
    drop(span);
  }

  #[napi]
  pub fn error(&self, log: String, span_name: String) {
    let span = tracing::span!(tracing::Level::ERROR, NAME_JS, "{}", span_name.as_str());
    span.in_scope(|| {
      tracing::error!("{}", log);
    });
    drop(span);
  }

  #[napi]
  pub fn warn(&self, log: String, span_name: String) {
    let span = tracing::span!(tracing::Level::WARN, NAME_JS, "{}", span_name.as_str());
    span.in_scope(|| {
      tracing::warn!("{}", log);
    });
    drop(span);
  }

  #[napi]
  pub fn trace(&self, log: String, span_name: String) {
    let span = tracing::span!(tracing::Level::TRACE, NAME_JS, "{}", span_name.as_str());
    span.in_scope(|| {
      tracing::trace!("{}", log);
    });
    drop(span);
  }
}
