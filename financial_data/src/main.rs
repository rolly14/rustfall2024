use serde::Deserialize;
use std::{fs, thread, time::Duration};
use ureq;  // Ensure ureq is in Cargo.toml

#[derive(Debug)]
struct Crypto {
    api_address: String,
    file_name: String,
    coin_type: CoinType,
}

#[derive(Debug)]
enum CoinType {
    Bitcoin,
    Ethereum,
}

#[derive(Debug)]
struct SP500 {
    api_address: String,
    file_name: String,
}

pub trait Pricing {
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self, price: f32);
}

// API response structure for deserialization (Bitcoin and Ethereum)
#[derive(Debug, Deserialize)]
struct CoinData {
    usd: f32,
}

// API response structure for S&P 500 
#[derive(Debug, Deserialize)]
struct SP500Data {
    chart: ChartData,
}

#[derive(Debug, Deserialize)]
struct ChartData {
    result: Vec<ResultData>,
}

#[derive(Debug, Deserialize)]
struct ResultData {
    meta: MetaData,
}

#[derive(Debug, Deserialize)]
struct MetaData {
    regularMarketPrice: f32,
}

impl Pricing for Crypto {
    fn fetch_price(&self) -> f32 {
        let response: Result<ureq::Response, ureq::Error> = ureq::get(&self.api_address).call();

        match response {
            Ok(res) => {
                match res.into_string() {
                    Ok(body) => {
                        println!("Raw API response: {}", body);
                        match serde_json::from_str::<serde_json::Value>(&body) {
                            Ok(data) => {
                                // Extract either Bitcoin or Ethereum price based on the coin type
                                let price = match self.coin_type {
                                    CoinType::Bitcoin => data["bitcoin"]["usd"].as_f64().unwrap_or(0.0) as f32,
                                    CoinType::Ethereum => data["ethereum"]["usd"].as_f64().unwrap_or(0.0) as f32,
                                };
                                println!("Fetched price: ${}", price);
                                return price;
                            }
                            Err(_) => {
                                eprintln!("Failed to parse API response JSON. Response body: {}", body);
                                return 0.0;
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("Failed to read the response body.");
                        return 0.0;
                    }
                }
            }
            Err(_) => {
                eprintln!("Failed to fetch price.");
                return 0.0;
            }
        }
    }

    fn save_to_file(&self, price: f32) {
        let coin_name = match self.coin_type {
            CoinType::Bitcoin => "Bitcoin",
            CoinType::Ethereum => "Ethereum",
        };
        let data = format!("{}: ${}\n", coin_name, price);
        fs::write(&self.file_name, data).expect("Unable to write to file");
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> f32 {
        let response: Result<ureq::Response, ureq::Error> = ureq::get(&self.api_address).call();

        match response {
            Ok(res) => {
                match res.into_string() {
                    Ok(body) => {
                        println!("Raw S&P 500 API response: {}", body);
                        match serde_json::from_str::<SP500Data>(&body) {
                            Ok(data) => {
                                let price = data.chart.result[0].meta.regularMarketPrice;
                                println!("Fetched S&P 500 price: {}", price);
                                return price;
                            }
                            Err(_) => {
                                eprintln!("Failed to parse S&P 500 price JSON. Response body: {}", body);
                                return 0.0;
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("Failed to read the S&P 500 response body.");
                        return 0.0;
                    }
                }
            }
            Err(_) => {
                eprintln!("Failed to fetch S&P 500 price.");
                return 0.0;
            }
        }
    }

    fn save_to_file(&self, price: f32) {
        let data = format!("S&P 500: {}\n", price);
        fs::write(&self.file_name, data).expect("Unable to write to file");
    }
}

fn main() {
    let btc_api = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string();
    let eth_api = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string();
    let sp500_api = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d".to_string(); 

    let btc_file = "btc_price.txt".to_string();
    let eth_file = "eth_price.txt".to_string();
    let sp500_file = "sp500_price.txt".to_string();

    // Create Crypto instances for Bitcoin and Ethereum
    let bitcoin = Crypto {
        api_address: btc_api,
        file_name: btc_file,
        coin_type: CoinType::Bitcoin, // Set the CoinType to Bitcoin
    };

    let ethereum = Crypto {
        api_address: eth_api,
        file_name: eth_file,
        coin_type: CoinType::Ethereum, // Set the CoinType to Ethereum
    };

    // Create SP500 instance as before
    let sp500 = SP500 {
        api_address: sp500_api,
        file_name: sp500_file,
    };

    loop {
        let btc_price = bitcoin.fetch_price();
        bitcoin.save_to_file(btc_price);

        let eth_price = ethereum.fetch_price();
        ethereum.save_to_file(eth_price);

        let sp500_price = sp500.fetch_price();
        sp500.save_to_file(sp500_price);

        thread::sleep(Duration::new(10, 0));  // Sleep for 10 seconds
    }
}
