use tracing_subscriber::{EnvFilter, fmt};

pub fn init_logging() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,proxy=debug"));

    fmt()
        .pretty() 
        .with_file(false)
        .with_target(false)
        .with_env_filter(filter)
        .init();
}
