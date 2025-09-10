use reqwest::blocking::Client;

///
/// # Fuzzer
/// holds the request methods
///
/// # Example
/// ```rust
/// # use fruf::Fuzzer;
/// # use reqwest::blocking::Client;
/// # use std::time::Duration;
/// # let client: Client = Client::builder()
/// #       .timeout(Duration::from_secs(10))
/// #       .user_agent("Rust-Fuzzer/1.0")
/// #       .build()
/// #       .unwrap();
/// Fuzzer::get_request("example", &client, "https://example.com/");
/// ```
///
pub struct Fuzzer;

impl Fuzzer {
    pub fn get_request(word: &str, client: &Client, base_url: &str) {
        let url: String = format!("{}{}", base_url, word);

        match client.get(&url).send() {
            Ok(response) => {
                let status = response.status();
                let length: u64 = response.content_length().unwrap_or(0);

                if status.is_success() || status.is_redirection() {
                    println!(
                        "{}\t[Status: {}, URL: {}] -> Size: {} bytes",
                        word, status, url, length
                    );
                } else {
                    println!(
                        "{}\t[Status: {}, URL: {}] -> Size: {} bytes",
                        word, status, url, length
                    );
                }
            }
            Err(e) => {
                // Only show errors if you want to
                if !e.is_timeout() {
                    eprintln!("Error for {}: {}", url, e);
                }
            }
        }
    }
}
