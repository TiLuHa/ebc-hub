use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ebc: HashMap<String, EbcConfig>,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct EbcConfig {
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub listen_address: String,
    pub port: u16,
}
