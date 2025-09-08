//!
//! fruf is a Web Fuzzer written in rust.
//! fruf short for fuzz ruffer u fool. ehh it might change
//!

use std::{fs, process::exit, sync::Arc, time::Duration};

use fruf::{parser, Client, ConfigApp, ThreadPool};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: ConfigApp = if let Ok(cfg) = parser() {
        cfg
    } else {
        std::process::exit(1);
    };

    //
    // check the wordlist existants.
    //
    if !fs::exists(&config.file_path)? {
        eprintln!("File Error: There is no file in the given path.");
        exit(1);
    }

    //
    // fetch the content of the wordlist
    //
    let file_content: Vec<String> = fs::read_to_string(&config.file_path)?
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.trim().is_empty())
        .collect();

    //
    // the Fuzzing logic keyword here.
    //
    // -----------------
    //
    // Client object that sends the requests
    //
    let client: Client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Rust-Fuzzer/1.0")
        .build()
        .unwrap();

    let shared_content: Arc<Vec<String>> = Arc::new(file_content);

    let pool: ThreadPool = ThreadPool::new(config.pool);

    for index in 0..shared_content.len() {
        let content_ref: Arc<Vec<String>> = Arc::clone(&shared_content);
        let client_ref: Client = client.clone();
        let base_url: String = config.url.clone();

        pool.execute(move || {
            // Get the word at the specific index
            let word = &content_ref[index];
            fuzzy_request(word, &client_ref, &base_url);
        });
    }

    Ok(())
}

fn fuzzy_request(word: &str, client: &Client, base_url: &str) {
    let url: String = format!("{}{}", base_url, word);

    match client.get(&url).send() {
        Ok(response) => {
            let status = response.status();
            let length: u64 = response.content_length().unwrap_or(0);

            if status.is_success() || status.is_redirection() {
                println!("[{}] {} -> {} bytes", status, url, length);
            } else {
                println!("[{}] {} -> {} bytes", status, url, length);
            }
        }
        Err(e) => {
            // Only show errors if you want to
            if !e.is_timeout() {
                eprintln!("Error for {}: {}", url, e);
            }
        }
    }
}
