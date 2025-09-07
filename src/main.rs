//!
//! fruf is a Web Fuzzer written in rust.
//! fruf short for fuzz ruffer u fool. ehh it might change
//!

use fruf::{parser, Client, ConfigApp, Response, ThreadPool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: ConfigApp = if let Ok(cfg) = parser() {
        cfg
    } else {
        std::process::exit(1);
    };

    // check the wordlist existants.
    if !std::fs::exists(&config.file_path)? {
        eprintln!("File Error: There is no file in the given path.");
        std::process::exit(1);
    }

    // fetch the content of the wordlist
    let _file_content: Vec<&str> = std::fs::read_to_string(&config.file_path)?
        .split("\n")
        .collect();

    // TODO : the Fuzzing logic keyword here.

    let pool: ThreadPool = ThreadPool::new(config.pool);

    pool.execute(|| todo!());

    // TODO : create thread pool to use
    let client: Client = Client::new();
    let resp: Response = client.get(config.url).send().await?;

    println!("Status: {}", resp.status());
    println!("Request Object : {:?}", resp);

    Ok(())
}

fn _fuzzy_request(_fuzz: &'static str) {
    todo!();
}
