//!
//! fruf is simple a Web Fuzzer written in rust.
//! fruf short for fuzz rustier u fool.
//!
//! trying my first projects :)
//!

use clap::Parser;
use reqwest::blocking::Client;
use std::{sync::Arc, time::Duration};

use fruf::{ConfigApp, ConfigAppBuilder, Fuzzer, ThreadPool};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialzing the configs
    let config: ConfigAppBuilder = ConfigAppBuilder::parse();

    let config: ConfigApp = ConfigApp::from(config);

    //
    // the Fuzzing logic keyword here.
    //
    // -----------------
    //
    // Client object that sends the requests
    //
    let client: Client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let shared_content: Arc<Vec<String>> = Arc::new(config.file_content);

    let pool: ThreadPool = ThreadPool::new(config.pool);

    for index in 0..shared_content.len() {
        let content_ref: Arc<Vec<String>> = Arc::clone(&shared_content);
        let client_ref: Client = client.clone();
        let base_url: String = config.url.clone();
        let header_ref: Vec<String> = config.headers.clone();
        let body_ref: String = config.body.clone();

        match config.method.as_str() {
            "GET" => {
                pool.execute(move || {
                    // Get the word at the specific index
                    let word: &String = &content_ref[index];
                    Fuzzer::get_request(word, &client_ref, &base_url, header_ref, body_ref);
                });
            }
            "POST" => {
                pool.execute(move || {
                    let word: &String = &content_ref[index];
                    Fuzzer::post_request(word, &client_ref, &base_url, header_ref, body_ref);
                });
            }
            other_method => {
                eprintln!(
                    "Argmuent Error: No method {} built in this tool.",
                    other_method
                );
                break;
            }
        }
    }

    Ok(())
}
