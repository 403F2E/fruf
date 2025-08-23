mod config;
mod utils;

use reqwest::{self, Response};
use std::process::exit;
use utils::args_handle;

use crate::config::ConfigApp;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let config: ConfigApp = if let Ok(config) = args_handle() {
        config
    } else {
        exit(1);
    };

    println!(
        "Socket Connection to : {:?}:{:?}",
        config.port.clone().unwrap(),
        config.addr.clone().unwrap(),
    );

    let link: String = format!("http://{}:{}", &config.addr.unwrap(), &config.port.unwrap());

    let resp: Response = reqwest::get(link).await?;

    println!("Status: {}", resp.status());
    println!("Body: {}", resp.text().await?);

    Ok(())
}
