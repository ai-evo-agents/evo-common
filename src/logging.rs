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
