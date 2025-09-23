//!
//! fruf is a Web Fuzzer written in rust.
//! fruf short for fuzz rustier u fool.
//!

use reqwest::blocking::Client;
use std::{sync::Arc, time::Duration};

use fruf::{parser, ConfigApp, Fuzzer, ThreadPool};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: ConfigApp = if let Ok(cfg) = parser() {
        cfg
    } else {
        std::process::exit(1);
    };

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

    let shared_content: Arc<Vec<String>> = Arc::new(config.file_content);

    let pool: ThreadPool = ThreadPool::new(config.pool);

    for index in 0..shared_content.len() {
        let content_ref: Arc<Vec<String>> = Arc::clone(&shared_content);
        let client_ref: Client = client.clone();
        let base_url: String = config.url.clone();
        let header_ref: Vec<String> = config.headers.clone();
        let _body_ref: Vec<String> = config.body.clone();

        pool.execute(move || {
            // Get the word at the specific index
            let word: &String = &content_ref[index];
            Fuzzer::get_request(word, &client_ref, &base_url, header_ref);
        });
    }

    Ok(())
}
