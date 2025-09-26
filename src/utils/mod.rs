mod fuzz;
mod parser;

pub use fuzz::*;
pub use parser::*;

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
