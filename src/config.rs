pub const FLAG_ADDR: [&'static str; 2] = ["-a", "--addr"];
pub const FLAG_PORT: [&'static str; 2] = ["-p", "--port"];
pub const FLAG_PATH: [&'static str; 2] = ["-w", "--wordlist"];
pub const DEFAULT_ADDR: &'static str = "127.0.0.1";
pub const DEFAULT_PORT: &'static str = "80";
pub const DEFAULT_PATH: &'static str = "/usr/share/fruf/wordlist.txt";

#[allow(dead_code)]
pub struct ConfigApp {
    pub addr: Option<String>,
    pub port: Option<String>,
    pub file_path: Option<String>,
}

impl ConfigApp {
    pub fn default_setup() -> Self {
        Self {
            addr: Some(DEFAULT_ADDR.to_owned()),
            port: Some(DEFAULT_PORT.to_owned()),
            file_path: Some(DEFAULT_PATH.to_owned()),
        }
    }
}
