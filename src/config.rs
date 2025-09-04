use crate::DEFAULT_PATH;
use std::path::PathBuf;

///
/// Config struct stores all the argument values given to the program
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConfigApp {
    /// URL to target (must start with http:// or https://)
    pub url: Option<String>,

    /// File Path to the Wordlist to Fuzz from
    pub file_path: PathBuf,

    /// Thread Pool size
    pub pool: u8,

    /// HTTP METHOD used for the request
    pub method: String,
}

impl ConfigApp {}

impl Default for ConfigApp {
    fn default() -> Self {
        Self {
            url: None,
            file_path: DEFAULT_PATH.into(),
            pool: 0,
            method: "GET".to_owned(),
        }
    }
}
