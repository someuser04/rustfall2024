use std::env;
use std::time::Duration;

pub struct Config {
    pub urls: Vec<String>,
    pub timeout: Duration,
    pub threads: usize,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let urls = args
            .next()
            .ok_or("No URLs provided. Provide a comma-separated list of URLs.")?
            .split(',')
            .map(String::from)
            .collect();

        let timeout = args
            .next()
            .and_then(|t| t.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(5));

        let threads = args
            .next()
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(4);

        Ok(Config { urls, timeout, threads })
    }
}
