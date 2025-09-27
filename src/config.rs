use clap::Parser;
use std::{fs, path::PathBuf, process::exit};

// the default wordlist used for the fuzzing
pub const DEFAULT_PATH: &'static str = "/usr/share/fruf/wordlist.txt";

// --- ConfigAppBuilder Region --- //

///
/// Config struct stores all the argument values given to the program
///
#[derive(Debug, Parser)]
#[command(
    version,
    about = "web fuzzer built in rust",
    long_about = "Fruf is a Web fuzzer built in rust. short for Fuzz Ruffer U Fool # ehh it definitly should change"
)]
pub struct ConfigAppBuilder {
    /// URL to target (must start with http:// or https://)
    #[arg(short = 'u', long = "url")]
    pub url: String,

    /// Header to send to the target
    #[arg(short = 'e', long = "header")]
    pub headers: Option<String>,

    /// Header to send to the target
    #[arg(short = 'b', long = "body")]
    pub body: Option<String>,

    /// File Path to the Wordlist to Fuzz from
    #[arg(short = 'w', long = "wordlist", default_value = DEFAULT_PATH)]
    pub file_path: PathBuf,

    /// Thread Pool size
    #[arg(short = 't', long = "thread", default_value_t = 25u8)]
    pub pool: u8,

    /// HTTP METHOD used for the request
    #[arg(short = 'm', long = "method", default_value = "GET")]
    pub method: String,
}

// --- ConfigAppBuilder Region ends --- //

// --- ConfigApp Region --- //

pub struct ConfigApp {
    pub url: String,
    pub headers: Vec<String>,
    pub body: String,
    pub file_content: Vec<String>,
    pub pool: usize,
    pub method: String,
}

impl From<ConfigAppBuilder> for ConfigApp {
    fn from(builder_config: ConfigAppBuilder) -> Self {
        //
        // check the wordlist existence.
        //
        if let Ok(exist) = fs::exists(&builder_config.file_path) {
            if !exist {
                eprintln!("File Error: There is no file in the given path.");
                exit(1);
            }
        }

        //
        // '--url' or '-u'.
        //
        // URL must starts with 'https://' or 'http://'.
        //
        if !builder_config.url.starts_with("https://") && !builder_config.url.starts_with("http://")
        {
            eprintln!("Argument Error: URL must start with 'http://' or 'https://'.");
            exit(1);
        }

        //
        // fetch the content of the wordlist
        //
        let file_content: Vec<String> = fs::read_to_string(&builder_config.file_path)
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.trim().is_empty())
            .collect();

        //
        // parse the headers into a vector of key-value strings
        //
        let headers: Vec<String> = if let Some(headers) = builder_config.headers {
            headers.split(";").map(|s| s.to_owned()).collect()
        } else {
            Vec::new()
        };

        //
        // '--method' or '-m'.
        //
        // METHOD must be an VALID HTTP method.
        //
        if !matches!(builder_config.method.as_str(), "GET" | "POST") {
            eprintln!("Argument Error: METHOD must be valid HTTP method.");
            exit(1);
        }

        //
        // no body should be given when the method is a GET request
        //
        let body: String = if let Some(body) = builder_config.body {
            body
        } else {
            String::new()
        };

        Self {
            url: builder_config.url,
            headers,
            body,
            file_content: file_content,
            pool: builder_config.pool as usize,
            method: builder_config.method,
        }
    }
}

// --- ConfigApp Region --- //
