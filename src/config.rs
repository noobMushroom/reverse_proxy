use std::time::Duration;

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

    /// Max object size to cache in mb 1 to 20 mb
    #[arg(short, long, value_name = "Max Object Size", value_parser = parse_max_object_size)]
    pub max_object_size: usize,

    /// Cache size to store in mb 50 to 500mb
    #[arg(short, long, value_name = "Cache Size", value_parser = parse_cache_size)]
    pub cache_size: usize,

    /// Time to live for the cache in seconds between 0 to 7200
    #[arg(long, value_name = "Ttl", value_parser = parse_ttl_time)]
    pub ttl: Duration,
}

fn parse_url(s: &str) -> Result<Url, String> {
    let url = Url::parse(s).map_err(|e| e.to_string())?;

    match url.scheme() {
        "http" | "https" => Ok(url),
        _ => Err("Only http/https allowed".into()),
    }
}

fn parse_max_object_size(input: &str) -> Result<usize, String> {
    let mb: f64 = input.parse().map_err(|_| "Invalid number".to_string())?;

    if !(1.0 <= mb && mb <= 20.0) {
        return Err("Max object size should be between 1 and 20 MB".into());
    }

    let bytes = mb * 1024.0 * 1024.0;

    Ok(bytes as usize)
}

fn parse_cache_size(input: &str) -> Result<usize, String> {
    let mb: f64 = input.parse().map_err(|_| "Invalid number".to_string())?;

    if !(50.0 <= mb && mb <= 500.0) {
        return Err("Max object size should be between 1 to 500 mb".into());
    }

    let bytes = mb * 1024.0 * 1024.0;

    Ok(bytes as usize)
}

fn parse_ttl_time(input: &str) -> Result<Duration, String> {
    let seconds: u64 = input.parse().map_err(|_| "Invalid number".to_string())?;

    if !(seconds <= 7200) {
        return Err("Max object size should be between 0 and 20 MB".into());
    }

    Ok(Duration::from_secs(seconds))
}
