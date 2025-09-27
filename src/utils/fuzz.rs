use reqwest::{blocking::Client, header::HeaderMap};

use crate::utils::{replace_fuzz, to_headermap};

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
    // the method of the fuzzer to execute the GET request
    pub fn get_request(
        word: &str,
        client: &Client,
        base_url: &str,
        headers: Vec<String>,
        body: String,
    ) {
        let (url, headers, body): (String, Vec<String>, String) =
            replace_fuzz(base_url, headers, body, word);

        // generate the header map object to send in the request
        let headers_map: HeaderMap = to_headermap(headers);

        match client.get(&url).headers(headers_map).body(body).send() {
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

    // the method of the fuzzer to execute the GET request
    pub fn post_request(
        word: &str,
        client: &Client,
        base_url: &str,
        headers: Vec<String>,
        body: String,
    ) {
        let (url, headers, body): (String, Vec<String>, String) =
            replace_fuzz(base_url, headers, body, word);

        // generate the header map object to send in the request
        let headers_map: HeaderMap = to_headermap(headers);

        match client.post(&url).headers(headers_map).body(body).send() {
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
