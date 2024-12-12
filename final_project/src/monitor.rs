use crate::config::Config;
use crate::reporter::WebsiteStatus;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
use ureq;

pub struct Monitor {
    config: Config,
}

impl Monitor {
    pub fn new(config: Config) -> Monitor {
        Monitor { config }
    }

    pub fn run(&self) {
        let (tx, rx) = mpsc::channel();

        let urls = self.config.urls.clone();
        let timeout = self.config.timeout;
        let threads = self.config.threads;

        let mut handles = Vec::new();

        for chunk in urls.chunks(threads) {
            let tx = tx.clone();
            let chunk = chunk.to_vec();

            let handle = thread::spawn(move || {
                for url in chunk {
                    let start = Instant::now();
                    let result = ureq::get(&url)
                        .timeout(timeout)
                        .call();

                    let status = match result {
                        Ok(response) => Ok(response.status()),
                        Err(err) => Err(err.to_string()),
                    };

                    let response_time = start.elapsed();

                    let website_status = WebsiteStatus {
                        url,
                        status,
                        response_time,
                        timestamp: chrono::Utc::now(),
                    };

                    tx.send(website_status).unwrap();
                }
            });

            handles.push(handle);
        }

        drop(tx);

        while let Ok(website_status) = rx.recv() {
            println!(
                "URL: {}, Status: {:?}, Response Time: {:?}, Timestamp: {}",
                website_status.url,
                website_status.status,
                website_status.response_time,
                website_status.timestamp
            );
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
} 
