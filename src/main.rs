//!
//! fruf is a Web Fuzzer written in rust.
//! fruf short for fuzz ruffer u fool. ehh it might change
//!

mod args;
mod config;

use reqwest::{Client, Response};

use config::ConfigApp;

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

    // TODO : the Fuzzing logic keyword here.

    // TODO : create thread pool to use
    let client: Client = Client::new();
    let resp: Response = client.get(config.url).send().await?;

    println!("Status: {}", resp.status());
    //println!("Body: {}", resp.text().await?);
    println!("Request Object : {:?}", resp);

    Ok(())
}
