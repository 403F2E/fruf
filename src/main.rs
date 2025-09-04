mod args;
mod config;

use fruf::{ConfigApp, DEFAULT_PATH};

use args::parser;
use reqwest::{self, Response};
use std::{fs, process::exit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: ConfigApp = if let Ok(cfg) = parser() {
        cfg
    } else {
        exit(1);
    };

    if !fs::exists(&config.file_path.clone())? {
        eprintln!("File Error: There is no file in the given path.");
        exit(1);
    }

    let file_content: String =
        if let Ok(file_content) = fs::read_to_string(&config.file_path.clone()) {
            file_content
        } else {
            eprintln!("File Error: Not able to open the file.");
            exit(1);
        };

    println!(
        "The content found in the file {:?} is : \n{}",
        config.file_path.clone(),
        file_content
    );

    // TODO : handle the Fuzz keyword here.

    let resp: Response;
    if let Some(url) = config.url {
        resp = reqwest::get(url).await?;
    } else {
        eprintln!("URL error: URL must have a value.");
        exit(1);
    };

    println!("Status: {}", resp.status());
    println!("Body: {}", resp.text().await?);

    Ok(())
}
