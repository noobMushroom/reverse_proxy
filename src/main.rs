use std::sync::Arc;

use clap::Parser;
use proxy_library::config::Cli;
use proxy_library::startup::run;
use proxy_library::telemetry::init_logging;

#[tokio::main]
async fn main() {
    init_logging();
    let cli = Arc::new(Cli::parse());
    if let Err(e) = run(cli).await {
        eprint!("Error: {}\n", e);
        std::process::exit(1)
    }
}
