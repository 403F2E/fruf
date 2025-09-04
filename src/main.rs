//!
//! fruf is a Web Fuzzer written in rust.
//! fruf short for fuzz ruffer u fool. ehh it might change
//!

mod args;

use reqwest::{self, Response};

use fruf::{ConfigApp, FLAG_PATH, FLAG_POOL, FLAG_URL};

use args::parser;
use std::{fs, process::exit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: ConfigApp = if let Ok(cfg) = parser() {
        cfg
    } else {
        exit(1);
    };

    if !fs::exists(&config.file_path)? {
        eprintln!("File Error: There is no file in the given path or the path not valid.");
        exit(1);
    }

    let file_content: String = if let Ok(file_content) = fs::read_to_string(&config.file_path) {
        file_content
    } else {
        eprintln!("File Error: Not able to open the file.");
        exit(1);
    };

    println!(
        "The content found in the file {:?} is : \n{}",
        config.file_path, file_content
    );

    // TODO : handle the Fuzz keyword here.

    // TODO : change the reqwest crate with native implementation using the TcpListener
    let resp: Response;
    if let Some(url) = config.url {
        resp = reqwest::get(url).await?;
    } else {
        eprintln!("URL error: a URL must be given.");
        exit(1);
    };

    println!("Status: {}", resp.status());
    println!("Body: {}", resp.text().await?);

    Ok(())
}
