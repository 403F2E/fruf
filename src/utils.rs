use crate::config::{DEFAULT_ADDR, DEFAULT_PORT, FLAG_ADDR, FLAG_PORT};
use std::env::args;

pub(crate) fn args_handle() -> Result<(String, String), ()> {
    let mut argv = args().into_iter();
    let mut addr: String = DEFAULT_ADDR.to_owned();
    let mut port: String = DEFAULT_PORT.to_owned();

    let _ = argv.next().unwrap();

    loop {
        let arg_flag: String = if let Some(arg_flag) = argv.next() {
            arg_flag
        } else {
            break;
        };

        match arg_flag.as_str() {
            addr_flag if FLAG_ADDR.contains(&addr_flag) => {
                let addr_value: String = if let Some(arg_value) = argv.next() {
                    arg_value
                } else {
                    break;
                };

                match addr_value.as_str() {
                    addr_arg if addr_arg.contains(".") => {
                        addr = addr_value.to_owned();
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
                match port_value.as_str() {
                    port_value if port_value.chars().all(|c| c.is_numeric()) => {
                        port = port_value.to_owned();
                    }
                    _ => {
                        eprintln!("Argument Error: Unknown value for --port");
                        return Err(());
                    }
                }
            }
            _ => {
                eprintln!("Argument Error: Unknown flag");
                return Err(());
            }
        }
    }

    Ok((addr, port))
}
