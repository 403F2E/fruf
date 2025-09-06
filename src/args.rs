use clap::Parser;

use crate::ConfigApp;

///
/// Argument Parser
///
/// # Usage:
/// - in the main.rs
/// ```
/// let config = args::parser().unwrap();
/// ```
//
/// # Notice:
/// After implementing more than 5 arguments the 'clap' crate is to be used.
///

pub fn parser() -> Result<ConfigApp, ()> {
    //let mut argv = args().skip(1);
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
