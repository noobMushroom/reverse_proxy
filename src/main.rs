use std::sync::Arc;

use clap::Parser;
use proxy_library::config::Cli;
use proxy_library::startup::run;

#[tokio::main]
async fn main() {
    let cli = Arc::new(Cli::parse());
    if let Err(e) = run(cli).await {
        eprint!("Error: {}\n", e);
        std::process::exit(1)
    }
}
