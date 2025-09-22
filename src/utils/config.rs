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
    pub body: Vec<String>,
    pub file_content: Vec<String>,
    pub pool: u8,
    pub method: String,
}

impl From<ConfigAppBuilder> for ConfigApp {
    fn from(builder_config: ConfigAppBuilder) -> Self {
        //
        //
        // check the wordlist existants.
        //
        if let Ok(exist) = fs::exists(&builder_config.file_path) {
            if !exist {
                eprintln!("File Error: There is no file in the given path.");
                exit(1);
            }
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

        let headers: Vec<String> = if let Some(headers) = builder_config.headers {
            vec![headers]
        } else {
            Vec::new()
        };

        let body: Vec<String> = if let Some(body) = builder_config.body {
            vec![body]
        } else {
            Vec::new()
        };

        Self {
            url: builder_config.url,
            headers,
            body,
            file_content: file_content,
            pool: builder_config.pool,
            method: builder_config.method,
        }
    }
}

// --- ConfigApp Region --- //
