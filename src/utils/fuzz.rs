use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};

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
    pub fn get_request(word: &str, client: &Client, base_url: &str, mut headers: Vec<String>) {
        let url: String;
        // replace the word FUZZ with the corresponding word from the list
        if !base_url.contains("FUZZ") {
            if !headers.is_empty() {
                headers = headers
                    .iter()
                    .map(|header| {
                        if header.contains("FUZZ") {
                            return header.replace("FUZZ", word);
                        }
                        header.to_string()
                    })
                    .collect();
            }
            url = base_url.to_owned();
        } else {
            url = base_url.replace("FUZZ", word);
        }

        let mut headers_map: HeaderMap = HeaderMap::new();

        for header in headers {
            if let Some((key, value)) = header.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                // Skip empty headers
                if key.is_empty() || value.is_empty() {
                    continue;
                }

                headers_map.insert(
                    HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    HeaderValue::from_str(value).unwrap(),
                );
            }
        }

        match client.get(&url).headers(headers_map).send() {
            Ok(response) => {
                let status = response.status();
                let length: u64 = response.content_length().unwrap_or(0);

                if status.is_informational()
                    || status.is_success()
                    || status.is_redirection()
                    || status.is_client_error()
                    || status.is_server_error()
                {
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
                if !e.is_timeout() {
                    eprintln!("Error for {}: {}", url, e);
                }
            }
        }
    }
}
