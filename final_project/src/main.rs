use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use ureq::Agent;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: u128,
    pub timestamp: u64,
}

pub fn monitor_websites(
    urls: Vec<String>,
    timeout_duration: u64,
    concurrent_limit: usize,
) -> Vec<WebsiteStatus> {
    let (tx, rx) = mpsc::channel();
    let agent = ureq::agent();
    let mut results = Vec::new();
    let mut threads = vec![];

    for url_batch in urls.chunks(concurrent_limit) {
        for url in url_batch {
            let tx = tx.clone();
            let url = url.clone();
            let agent = agent.clone();
            threads.push(thread::spawn(move || {
                monitor(url, tx, timeout_duration, agent);
            }));
        }
        for _ in 0..url_batch.len() {
            if let Ok(status) = rx.recv() {
                results.push(status);
            }
        }
    }

    for t in threads {
        let _ = t.join();
    }

    results
}

fn monitor(url: String, tx: mpsc::Sender<WebsiteStatus>, timeout_duration: u64, agent: Agent) {
    let start_time = Instant::now();
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    match agent.get(&url).timeout(Duration::from_secs(timeout_duration)).call() {
        Ok(res) => {
            let status = create_status(url.clone(), Ok(res.status()), start_time.elapsed().as_millis(), timestamp);
            let _ = tx.send(status);
        },
        Err(err) => {
            let error_message = handle_error(err);
            let status = create_status(url.clone(), Err(error_message), start_time.elapsed().as_millis(), timestamp);
            let _ = tx.send(status);
        }
    }
}

fn create_status(url: String, status: Result<u16, String>, response_time: u128, timestamp: u64) -> WebsiteStatus {
    WebsiteStatus {
        url,
        status,
        response_time,
        timestamp,
    }
}

fn handle_error(err: ureq::Error) -> String {
    match err {
        ureq::Error::Status(code, _) => format!("HTTP Error: {}", code),
        ureq::Error::Transport(transport_error) => match transport_error.kind() {
            ureq::ErrorKind::Io => "I/O Error".to_string(),
            ureq::ErrorKind::Dns => "DNS Error".to_string(),
            ureq::ErrorKind::ConnectionFailed => "Connection Failed".to_string(),
            _ => "Unknown Transport Error".to_string(),
        },
    }
}

fn main() {
    let urls = vec![
        "https://github.com/".to_string(),
        "https://unity.com/".to_string(),
        "https://www.unrealengine.com/en-US".to_string(),
        "https://replit.com/".to_string(),
        "https://play.rust-lang.org/?version=stable&mode=debug&edition=2021".to_string(),
        "https://www.w3schools.com/".to_string(),
        "https://stackoverflow.com/".to_string(),
        "https://learnpython.com/".to_string(),
        "https://developer.mozilla.org/en-US/".to_string(),
        "https://www.programiz.com/".to_string(),
        "https://www.geeksforgeeks.org/".to_string(),
        "https://cplusplus.com/".to_string(),
        "https://www.codecademy.com/".to_string(),
        "https://code.org/".to_string(),
        "https://www.freecodecamp.org/".to_string(),
        "https://codehs.com/".to_string(),
        "https://www.khanacademy.org/".to_string(),
        "https://www.hackerrank.com/".to_string(),
        "https://www.atlassian.com/".to_string(),
        "https://www.python.org/".to_string(),
        "https://getbootstrap.com/".to_string(),
        "https://www.lua.org/".to_string(),
        "https://www.javascript.com/".to_string(),
        "https://ubuntu.com/".to_string(),
        "https://javascript.info/".to_string(),
        "https://isocpp.org/".to_string(),
        "https://www.onlinegdb.com/".to_string(),
        "https://www.ruby-lang.org/en/".to_string(),
        "https://www.tutorialspoint.com/index.htm".to_string(),
        "https://rubyonrails.org/".to_string(),
        "https://www.scoutapm.com/".to_string(),
        "https://www.google.com/".to_string(),
        "https://www.pokemon.com/us".to_string(),
        "https://www.pokemoncenter.com/".to_string(),
        "https://www.square-enix-games.com/en_US/home".to_string(),
        "https://na.store.square-enix-games.com/".to_string(),
        "https://atlus.com/".to_string(),
        "https://www.sega.com/".to_string(),
        "https://www.nintendo.com/us/".to_string(),
        "https://www.capcomusa.com/".to_string(),
        "https://www.walmart.com/".to_string(),
        "https://www.target.com/".to_string(),
        "https://www.bestbuy.com/".to_string(),
        "https://www.amazon.com/ref=nav_logo".to_string(),
        "https://www.gamestop.com/".to_string(),
        "https://store.crunchyroll.com/".to_string(),
        "https://www.goodsmileus.com/".to_string(),
        "https://pcpartpicker.com/".to_string(),
        "https://www.amd.com/en.html".to_string(),
        "https://www.nvidia.com/en-us/".to_string(),

    ];
    
    let timeout_duration = 5; // Timeout in seconds
    let concurrent_limit = 50; // Limit concurrency

    let results = monitor_websites(urls, timeout_duration, concurrent_limit);

    for status in results {
        println!("URL: {}", status.url);
        println!("Timestamp: {}", status.timestamp);
        match status.status {
            Ok(code) => println!("Status: {} Response Time: {} ms", code, status.response_time),
            Err(err) => println!("Error: {} Response Time: {} ms", err, status.response_time),
        }
        println!(); // Blank line for readability
    }
}
