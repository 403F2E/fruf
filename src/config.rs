pub(crate) const FLAG_ADDR: [&'static str; 2] = ["-a", "--addr"];
pub(crate) const FLAG_PORT: [&'static str; 2] = ["-p", "--port"];
pub(crate) const DEFAULT_ADDR: &'static str = "127.0.0.1";
pub(crate) const DEFAULT_PORT: &'static str = "80";

#[allow(dead_code)]
pub(crate) struct AppConfig {
    addr: &'static str,
    port: &'static str,
    path: &'static str,
}
