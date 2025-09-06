use clap::Parser;
use std::path::PathBuf;

// the default wordlist used for the fuzzing
pub const DEFAULT_PATH: &'static str = "/usr/share/fruf/wordlist.txt";

///
/// Config struct stores all the argument values given to the program
///
#[derive(Debug, Parser)]
#[command(
    version,
    about = "web fuzzer built in rust",
    long_about = "Fruf is a Web fuzzer built in rust. short for Fuzz Ruffer U Fool # ehh it definitly should change"
)]
pub struct ConfigApp {
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
    #[arg(short = 'w', long = "wordlist", default_value=DEFAULT_PATH)]
    pub file_path: PathBuf,

    /// Thread Pool size
    #[arg(short = 't', long = "thread", default_value_t = 25u8)]
    pub pool: u8,

    /// HTTP METHOD used for the request
    #[arg(short = 'm', long = "method", default_value = "GET")]
    pub method: String,
}
