use clap::Parser;
use url::Url;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Website for which reverse proxy is needed
    #[arg(short, long, value_name = "Target website", value_parser = parse_url)]
    pub target: Url,

    /// Port at which to start
    #[arg(short, long, value_name = "Port")]
    pub port: usize,

    /// Max object size to cache in mb
    #[arg(short, long, value_name = "Max Object Size")]
    pub max_object_size: u8,

}

fn parse_url(s: &str) -> Result<Url, String> {
    let url = Url::parse(s).map_err(|e| e.to_string())?;

    match url.scheme() {
        "http" | "https" => Ok(url),
        _ => Err("Only http/https allowed".into()),
    }
}
