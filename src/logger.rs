use napi_derive::napi;

const NAME_JS: &str = "NylonJS";

#[napi]
pub struct Logger {
    span_name: Option<String>,
}

#[napi]
pub enum Level {
    Info,
    Debug,
    Error,
    Warn,
    Trace,
}

impl Level {
    fn to_tracing_level(&self) -> tracing::Level {
        match self {
            Level::Info => tracing::Level::INFO,
            Level::Debug => tracing::Level::DEBUG,
            Level::Error => tracing::Level::ERROR,
            Level::Warn => tracing::Level::WARN,
            Level::Trace => tracing::Level::TRACE,
        }
    }
}

#[napi]
impl Logger {
    #[napi(constructor)]
    pub fn new(level: Level) -> Self {
        // Setup tracing
        let level = level.to_tracing_level();
        let subscriber = tracing_subscriber::fmt().with_max_level(level).finish();
        match tracing::subscriber::set_global_default(subscriber) {
            Ok(_) => (),
            Err(_) => (),
        }
        tracing::info!("Starting Nylon application...");
        Logger { span_name: None }
    }

    #[napi]
    pub fn scope(&self, span_name: String) -> Logger {
        Logger {
            span_name: Some(span_name),
        }
    }

    fn get_span_name(&self, span_name: Option<String>) -> String {
        match &self.span_name {
            Some(s) => s.clone(),
            None => match span_name {
                Some(s) => s,
                None => "".to_string(),
            },
        }
    }

    #[napi]
    pub fn info(&self, log: String, span_name: Option<String>) {
        let s_name = self.get_span_name(span_name);
        let span = tracing::span!(tracing::Level::INFO, NAME_JS, "{}", s_name.as_str());
        span.in_scope(|| {
            tracing::info!("{}", log);
        });
        drop(span);
    }

    #[napi]
    pub fn debug(&self, log: String, span_name: Option<String>) {
        let s_name = self.get_span_name(span_name);
        let span = tracing::span!(tracing::Level::DEBUG, NAME_JS, "{}", s_name.as_str());
        span.in_scope(|| {
            tracing::debug!("{}", log);
        });
        drop(span);
    }

    #[napi]
    pub fn error(&self, log: String, span_name: Option<String>) {
        let s_name = self.get_span_name(span_name);
        let span = tracing::span!(tracing::Level::ERROR, NAME_JS, "{}", s_name.as_str());
        span.in_scope(|| {
            tracing::error!("{}", log);
        });
        drop(span);
    }

    #[napi]
    pub fn warn(&self, log: String, span_name: Option<String>) {
        let s_name = self.get_span_name(span_name);
        let span = tracing::span!(tracing::Level::WARN, NAME_JS, "{}", s_name.as_str());
        span.in_scope(|| {
            tracing::warn!("{}", log);
        });
        drop(span);
    }

    #[napi]
    pub fn trace(&self, log: String, span_name: Option<String>) {
        let s_name = self.get_span_name(span_name);
        let span = tracing::span!(tracing::Level::TRACE, NAME_JS, "{}", s_name.as_str());
        span.in_scope(|| {
            tracing::trace!("{}", log);
        });
        drop(span);
    }
}
