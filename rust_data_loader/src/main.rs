use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

// Struct definitions
#[derive(Debug, Deserialize)]
struct Bitcoin {
    price: f64,
}

#[derive(Debug, Deserialize)]
struct Ethereum {
    price: f64,
}

#[derive(Debug, Deserialize)]
struct SP500 {
    price: f64,
}

// Trait for standardized operations
trait Pricing {
    fn fetch_price(&self) -> Result<f64, String>;
    fn save_to_file(&self, price: f64, filename: &str) -> Result<(), String>;
}

// Implementations for each asset
impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, String> {
        let url = "https://api.coindesk.com/v1/bpi/currentprice/BTC.json";
        let response: serde_json::Value = serde_json::from_reader(
            ureq::get(url)
                .call()
                .map_err(|e| e.to_string())?
                .into_reader(),
        )
        .map_err(|e| e.to_string())?;
        response["bpi"]["USD"]["rate_float"]
            .as_f64()
            .ok_or_else(|| "Failed to parse Bitcoin price".to_string())
    }

    fn save_to_file(&self, price: f64, filename: &str) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .map_err(|e| e.to_string())?;
        writeln!(file, "Bitcoin price: ${:.2}", price).map_err(|e| e.to_string())
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, String> {
        let url = "https://api.coincap.io/v2/assets/ethereum";
        let response: serde_json::Value = serde_json::from_reader(
            ureq::get(url)
                .call()
                .map_err(|e| e.to_string())?
                .into_reader(),
        )
        .map_err(|e| e.to_string())?;
        response["data"]["priceUsd"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| "Failed to parse Ethereum price".to_string())
    }

    fn save_to_file(&self, price: f64, filename: &str) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .map_err(|e| e.to_string())?;
        writeln!(file, "Ethereum price: ${:.2}", price).map_err(|e| e.to_string())
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, String> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d";
        let response: serde_json::Value = serde_json::from_reader(
            ureq::get(url)
                .call()
                .map_err(|e| e.to_string())?
                .into_reader(),
        )
        .map_err(|e| e.to_string())?;
        response["chart"]["result"][0]["meta"]["regularMarketPrice"]
            .as_f64()
            .ok_or_else(|| "Failed to parse S&P 500 price".to_string())
    }

    fn save_to_file(&self, price: f64, filename: &str) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .map_err(|e| e.to_string())?;
        writeln!(file, "S&P 500 price: ${:.2}", price).map_err(|e| e.to_string())
    }
}

// Main execution loop
fn main() {
    let bitcoin = Bitcoin { price: 0.0 };
    let ethereum = Ethereum { price: 0.0 };
    let sp500 = SP500 { price: 0.0 };

    let filenames = vec![
        ("bitcoin.txt", &bitcoin as &dyn Pricing),
        ("ethereum.txt", &ethereum as &dyn Pricing),
        ("sp500.txt", &sp500 as &dyn Pricing),
    ];

    loop {
        for (filename, asset) in &filenames {
            let price = asset.fetch_price();
            match price {
                Ok(p) => {
                    if let Err(e) = asset.save_to_file(p, filename) {
                        eprintln!("Error saving data to {}: {}", filename, e);
                    }
                }
                Err(e) => eprintln!("Error fetching data: {}", e),
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}
