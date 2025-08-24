mod config;
mod utils;

use reqwest::{self, Response};
use std::{fs, process::exit};
use utils::args_handle;

use crate::config::ConfigApp;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let config: ConfigApp = if let Ok(config) = args_handle() {
        config
    } else {
        exit(1);
    };

    let file_content: String = if fs::exists(&config.file_path.clone().unwrap()).unwrap() {
        if let Ok(file_content) = fs::read_to_string(&config.file_path.clone().unwrap()) {
            file_content
        } else {
            eprintln!("File Error: Not able to open the file.");
            exit(1);
        }
    } else {
        eprintln!("File Error: There is no file in the given path.");
        exit(1);
    };

    println!(
        "The content found in the file {:?} is : \n{}",
        config.file_path.clone().unwrap(),
        file_content
    );

    println!(
        "Socket Connection established to : {:?}:{:?}",
        config.addr.clone().unwrap(),
        config.port.clone().unwrap(),
    );

    let link: String = format!("http://{}:{}", &config.addr.unwrap(), &config.port.unwrap());

    let resp: Response = reqwest::get(link).await?;

    println!("Status: {}", resp.status());
    println!("Body: {}", resp.text().await?);

    Ok(())
}
