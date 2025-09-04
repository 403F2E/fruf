// the project available arguments
pub const FLAG_URL: [&'static str; 2] = ["-u", "--url"];
pub const FLAG_PATH: [&'static str; 2] = ["-w", "--wordlist"];
pub const FLAG_POOL: [&'static str; 2] = ["-t", "--thread"];

// the default wordlist used for the fuzzing
pub const DEFAULT_PATH: &'static str = "/usr/share/fruf/wordlist.txt";

mod config;
pub mod request;

pub use config::*;
pub use request::*;
