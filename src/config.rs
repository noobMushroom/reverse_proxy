use clap::Parser;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli{
    /// Website for which reverse proxy is needed
    #[arg(short, long, value_name = "Target website")]
    pub target: String,

    /// Port at which to start
    #[arg(short, long, value_name = "Port")]
    pub port: usize
}
