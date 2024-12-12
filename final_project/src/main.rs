mod config;
mod monitor;
mod reporter;

use config::Config;
use monitor::Monitor;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        process::exit(1);
    });

    let monitor = Monitor::new(config);
    monitor.run();
}
