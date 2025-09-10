use clap::Parser;

use crate::ConfigApp;

///
/// # Argument parser
/// checks validity of the URL and the Method
///

pub fn parser() -> Result<ConfigApp, ()> {
    let config: ConfigApp = ConfigApp::parse();

    //
    // '--url' or '-u'.
    //
    // URL must starts with 'https://' or 'http://'.
    //
    if !config.url.starts_with("https://") && !config.url.starts_with("http://") {
        eprintln!("Argument Error: URL must start with 'http://' or 'https://'.");
        return Err(());
    }

    //
    // '--method' or '-m'.
    //
    // METHOD must be an VALID HTTP method.
    //
    if !is_http_valid(&config.method) {
        eprintln!("Argument Error: METHOD must be valid HTTP method.");
        return Err(());
    }

    Ok(config)
}

fn is_http_valid(method: &str) -> bool {
    matches!(method, "GET" | "POST" | "HEAD" | "OPTION" | "CONNECT")
}
