mod fuzz;

pub use fuzz::*;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

fn to_headermap(headers: Vec<String>) -> HeaderMap {
    let mut header_map: HeaderMap = HeaderMap::new();

    for header in headers {
        if let Some((key, value)) = header.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            // Skip empty headers
            if key.is_empty() || value.is_empty() {
                continue;
            }

            header_map.insert(
                HeaderName::from_bytes(key.as_bytes()).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
        }
    }

    header_map
}

fn replace_fuzz(
    base_url: &str,
    mut headers: Vec<String>,
    mut body: String,
    word: &str,
) -> (String, Vec<String>, String) {
    // replace the word FUZZ with the corresponding word from the list
    let url: String = base_url.replace("FUZZ", word);
    if !headers.is_empty() {
        headers = headers
            .iter()
            .map(|header| header.replace("FUZZ", word))
            .collect();
    }
    if !body.is_empty() {
        body = body.replace("FUZZ", word);
    }
    (url, headers, body)
}
