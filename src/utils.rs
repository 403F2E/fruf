use crate::config::{ConfigApp, FLAG_ADDR, FLAG_PATH, FLAG_PORT, FLAG_URL};
use std::env::args;

fn is_valid_ipv4_octet(octet: &str) -> bool {
    if let Ok(num) = octet.parse::<u8>() {
        num <= u8::MAX
    } else {
        false
    }
}

fn check_ipv4_pattern(ipv4: &str) -> bool {
    let bytes: Vec<&str> = ipv4.split(".").collect();

    if bytes.len() == 4 {
        for byte in bytes {
            if !is_valid_ipv4_octet(byte) {
                return false;
            }
        }
    }

    return true;
}

pub fn args_handle() -> Result<ConfigApp, ()> {
    let mut argv = args().into_iter();
    let mut config: ConfigApp = ConfigApp::default_setup();

    let _ = argv.next().unwrap();

    loop {
        let arg_flag: String = if let Some(arg_flag) = argv.next() {
            arg_flag
        } else {
            break;
        };

        match arg_flag.as_str() {
            url_flag if FLAG_URL.contains(&url_flag) => {
                let url_value: String = if let Some(arg_value) = argv.next() {
                    arg_value
                } else {
                    break;
                };

                match url_value {
                    url_value if check_ipv4_pattern(&url_value) => {
                        config.url = Some(url_value.to_owned());
                    }
                    _ => {
                        eprintln!("Argument Error: Unknown value for flag --url.");
                        return Err(());
                    }
                }
            }

            addr_flag if FLAG_ADDR.contains(&addr_flag) => {
                let addr_value: String = if let Some(arg_value) = argv.next() {
                    arg_value
                } else {
                    break;
                };

                match addr_value {
                    addr_value if check_ipv4_pattern(&addr_value) => {
                        config.addr = Some(addr_value.to_owned());
                    }
                    _ => {
                        eprintln!("Argument Error: Unknown value for flag --addr.");
                        return Err(());
                    }
                }
            }
            port_flag if FLAG_PORT.contains(&port_flag) => {
                let port_value: String = if let Some(arg_value) = argv.next() {
                    arg_value
                } else {
                    break;
                };
                match port_value {
                    port_value if port_value.chars().all(|c| c.is_numeric()) => {
                        config.port = Some(port_value.to_owned());
                    }
                    _ => {
                        eprintln!("Argument Error: Unknown value for --port.");
                        return Err(());
                    }
                }
            }
            file_flag if FLAG_PATH.contains(&file_flag) => {
                let path_value: String = if let Some(arg_value) = argv.next() {
                    arg_value
                } else {
                    break;
                };
                match path_value {
                    path_value
                        if path_value
                            .chars()
                            .all(|c| c.is_ascii_alphabetic() || c == '/') =>
                    {
                        config.file_path = Some(path_value.to_owned());
                    }
                    _ => {
                        eprintln!("Argument Error: This is not the format of an ipv4.");
                        return Err(());
                    }
                }
            }
            _ => {
                eprintln!("Argument Error: Unknown flag.");
                return Err(());
            }
        }
    }

    Ok(config)
}
