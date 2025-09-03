pub const FLAG_URL: [&'static str; 2] = ["-u", "--url"];
pub const FLAG_PATH: [&'static str; 2] = ["-w", "--wordlist"];
pub const FLAG_POOL: [&'static str; 2] = ["-t", "--thread"];

pub const DEFAULT_PATH: &'static str = "/usr/share/fruf/wordlist.txt";

mod config;

pub use config::*;
