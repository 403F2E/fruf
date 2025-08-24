pub const FLAG_ADDR: [&'static str; 2] = ["-a", "--addr"];
pub const FLAG_PORT: [&'static str; 2] = ["-p", "--port"];
pub const FLAG_URL: [&'static str; 2] = ["-u", "--url"];
pub const FLAG_PATH: [&'static str; 2] = ["-w", "--wordlist"];
pub const DEFAULT_URL: &'static str = "http://127.0.0.1:80";
pub const DEFAULT_PATH: &'static str = "/usr/share/fruf/wordlist.txt";

#[derive(Debug, Clone)]
pub struct ConfigApp {
    pub addr: Option<String>,
    pub port: Option<String>,
    pub url: Option<String>,
    pub file_path: Option<String>,
}

impl ConfigApp {
    pub fn default_setup() -> Self {
        Self {
            addr: None,
            port: None,
            url: Some(DEFAULT_URL.to_owned()),
            file_path: Some(DEFAULT_PATH.to_owned()),
        }
    }
}
