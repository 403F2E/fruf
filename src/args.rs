use crate::{ConfigApp, FLAG_PATH, FLAG_POOL, FLAG_URL};
use std::env::args;

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
    let mut argv = args().skip(1);
    let mut config: ConfigApp = ConfigApp::default();

    while let Some(arg_flag) = argv.next() {
        match arg_flag.as_str() {
            //
            // # First Pattern:
            //
            // '--url' or '-u'.
            //
            // Check if the --url flag own a value:
            //
            // - if true
            //     the URL must starts with 'https://' or 'http://'.
            // - if not
            //     return Err
            //
            url_flag if FLAG_URL.contains(&url_flag) => {
                let url_value: String = argv.next().ok_or_else(|| {
                    eprintln!("Argument Error: the '--url' flag requires a url value.");
                })?;

                if !url_value.starts_with("https://") && !url_value.starts_with("http://") {
                    eprintln!("Argument Error: URL must start with 'http://' or 'https://'.");
                    return Err(());
                }

                config.url = Some(url_value);
            }

            //
            // # Second Pattern:
            //
            // '--method' or '-m'.
            //
            // Check if the --method flag own a value:
            //
            // - if true
            //     the METHOD must be an VALID HTTP method.
            // - if not
            //     return Err
            //
            method_flag if FLAG_URL.contains(&method_flag) => {
                let method_value: String = argv.next().ok_or_else(|| {
                    eprintln!("Argument Error: the '--method' flag requires a url value.");
                })?;

                if !is_http_valid(&method_value) {
                    eprintln!("Argument Error: METHOD must be valid HTTP method.");
                    return Err(());
                }

                config.method = method_value;
            }

            //
            // # Second Pattern:
            //
            // '--wordlist' or '-w'.
            //
            // Check if the --wordlist flag own a value:
            //
            // - if true
            //     FILE PATH mustn't be empty.
            // - if not
            //     return Err
            //
            file_flag if FLAG_PATH.contains(&file_flag) => {
                let path_value: String = argv.next().ok_or_else(|| {
                    eprintln!("Argument Error: the '--wordlist' flag requires a file path.")
                })?;

                if path_value.is_empty() {
                    eprintln!("Argument Error: file path cannot be empty.");
                    return Err(());
                }

                config.file_path = path_value.into();
            }
            //
            // # Third Pattern:
            //
            // '--thread' or '-t'.
            //
            // Check if the --thread flag own a value:
            //
            // - if true
            //     POOL SIZE must be an integer.
            // - if not
            //     return Err
            //
            pool_flag if FLAG_POOL.contains(&pool_flag) => {
                let pool_value: String = argv.next().ok_or_else(|| {
                    eprintln!("Argument Error: the '--thread' requires an integer.")
                })?;

                config.pool = if let Ok(pool) = pool_value.parse::<u8>() {
                    pool
                } else {
                    eprintln!("Argument Error: POOL NUMBER value must be an valid number.");
                    return Err(());
                }
            }
            //
            // # if not of the above return err
            //
            _ => {
                eprintln!("Argument Error: Unknown flag {}.", arg_flag);
                return Err(());
            }
        }
    }

    Ok(config)
}

fn is_http_valid(method: &str) -> bool {
    matches!(method, "GET" | "POST" | "HEAD" | "OPTION" | "CONNECT")
}
