use serde::Deserialize;
use std::{fs, thread, time::Duration};
use ureq;

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

#[derive(Debug, Deserialize)]
struct Bitcoin {
    bitcoin: CoinData,
}

#[derive(Debug, Deserialize)]
struct Ethereum {
    ethereum: CoinData,
}

#[derive(Debug, Deserialize)]
struct CoinData {
    usd: f32,
}

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
        match self.coin_type {
            CoinType::Bitcoin | CoinType::Ethereum => {
                let response: Result<ureq::Response, ureq::Error> = ureq::get(&self.api_address).call();

                match response {
                    Ok(res) => {
                        match res.into_string() {
                            Ok(body) => {
                                println!("Raw API response: {}", body);
                                match self.coin_type {
                                    CoinType::Bitcoin => {
                                        match serde_json::from_str::<Bitcoin>(&body) {
                                            Ok(data) => {
                                                println!("Fetched Bitcoin price: ${}", data.bitcoin.usd);
                                                data.bitcoin.usd
                                            }
                                            Err(e) => {
                                                eprintln!("Failed to parse Bitcoin API response JSON: {}", e);
                                                0.0
                                            }
                                        }
                                    }
                                    CoinType::Ethereum => {
                                        match serde_json::from_str::<Ethereum>(&body) {
                                            Ok(data) => {
                                                println!("Fetched Ethereum price: ${}", data.ethereum.usd);
                                                data.ethereum.usd
                                            }
                                            Err(e) => {
                                                eprintln!("Failed to parse Ethereum API response JSON: {}", e);
                                                0.0
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to read the response body: {}", e);
                                0.0
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch price: {}", e);
                        0.0
                    }
                }
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
                                price
                            }
                            Err(e) => {
                                eprintln!("Failed to parse S&P 500 price JSON: {}", e);
                                0.0
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read the S&P 500 response body: {}", e);
                        0.0
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch S&P 500 price: {}", e);
                0.0
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

    let bitcoin = Crypto {
        api_address: btc_api,
        file_name: btc_file,
        coin_type: CoinType::Bitcoin,
    };

    let ethereum = Crypto {
        api_address: eth_api,
        file_name: eth_file,
        coin_type: CoinType::Ethereum,
    };

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

        thread::sleep(Duration::new(10, 0));
    }
}