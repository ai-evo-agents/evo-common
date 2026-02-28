use std::env;
use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

const DEFAULT_LOG_DIR: &str = "./logs";
const ENV_LOG_DIR: &str = "EVO_LOG_DIR";

pub fn log_dir() -> PathBuf {
    env::var(ENV_LOG_DIR)
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_LOG_DIR))
}

pub fn init_logging(component: &str) -> WorkerGuard {
    let dir = log_dir();
    std::fs::create_dir_all(&dir).expect("Failed to create log directory");

    let file_appender = tracing_appender::rolling::daily(&dir, format!("{component}.log"));
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    let stdout_layer = fmt::layer().with_target(true).with_thread_ids(false);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .with(stdout_layer)
        .init();

    guard
}

// ─── OpenTelemetry integration (behind "tracing-otel" feature) ────────────────

#[cfg(feature = "tracing-otel")]
pub struct OtelGuard {
    provider: opentelemetry_sdk::trace::SdkTracerProvider,
}

#[cfg(feature = "tracing-otel")]
impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(e) = self.provider.shutdown() {
            eprintln!("OpenTelemetry shutdown error: {e}");
        }
    }
}

/// Initialise structured logging **with** an OpenTelemetry tracing layer.
///
/// Spans produced by the `tracing` crate are forwarded to the given OTLP HTTP
/// endpoint (e.g. `http://localhost:3300`) as distributed traces.  The
/// `component` name is used both as the log-file stem and as the OTel
/// `service.name` resource attribute.
///
/// Returns two guards that **must** be held for the process lifetime:
/// * `WorkerGuard` – flushes the non-blocking file appender on drop.
/// * `OtelGuard`   – shuts down the tracer provider on drop.
#[cfg(feature = "tracing-otel")]
pub fn init_logging_with_otel(component: &str, otlp_endpoint: &str) -> (WorkerGuard, OtelGuard) {
    use opentelemetry::global;
    use opentelemetry::trace::TracerProvider;
    use opentelemetry_otlp::{SpanExporter, WithExportConfig};
    use opentelemetry_sdk::Resource;
    use opentelemetry_sdk::propagation::TraceContextPropagator;
    use opentelemetry_sdk::trace::SdkTracerProvider;
    use tracing_opentelemetry::OpenTelemetryLayer;

    // W3C Trace-Context propagator (traceparent / tracestate headers)
    global::set_text_map_propagator(TraceContextPropagator::new());

    // OTLP HTTP span exporter – the SDK appends `/v1/traces` automatically
    let exporter = SpanExporter::builder()
        .with_http()
        .with_endpoint(otlp_endpoint)
        .build()
        .expect("Failed to build OTLP span exporter");

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(
            Resource::builder()
                .with_service_name(component.to_owned())
                .build(),
        )
        .build();

    global::set_tracer_provider(provider.clone());

    let otel_layer = OpenTelemetryLayer::new(provider.tracer(component.to_owned()));

    // File + stdout layers (identical to `init_logging`)
    let dir = log_dir();
    std::fs::create_dir_all(&dir).expect("Failed to create log directory");
    let file_appender = tracing_appender::rolling::daily(&dir, format!("{component}.log"));
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    let stdout_layer = fmt::layer().with_target(true).with_thread_ids(false);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .with(stdout_layer)
        .with(otel_layer)
        .init();

    (guard, OtelGuard { provider })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_log_dir() {
        unsafe { env::remove_var(ENV_LOG_DIR) };
        assert_eq!(log_dir(), PathBuf::from("./logs"));
    }

    #[test]
    fn custom_log_dir() {
        unsafe { env::set_var(ENV_LOG_DIR, "/tmp/evo-test-logs") };
        assert_eq!(log_dir(), PathBuf::from("/tmp/evo-test-logs"));
        unsafe { env::remove_var(ENV_LOG_DIR) };
    }
}
