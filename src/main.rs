mod config;
mod utils;

use reqwest::{self, Response};
use std::process::exit;
use utils::args_handle;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let (addr, port): (String, String) = if let Ok((addr, port)) = args_handle() {
        (addr, port)
    } else {
        exit(1);
    };

    println!("Socket Connection to : {}:{}", addr, port);

    let link: String = format!("http://{}:{}", addr, port);

    let resp: Response = reqwest::get(link).await?;

    println!("Status: {}", resp.status());
    println!("Body: {}", resp.text().await?);

    Ok(())
}
