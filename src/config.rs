use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub simulation: bool,
    pub debug_mode: bool,
    pub max_retries: u8,
    pub check_interval: f32,
    pub purchase_amount_usdt: f64,
    pub min_required_quantity: f64,
    pub profit_target_percent: f64,
    pub use_websocket: bool,
    pub telegram_enabled: bool,
    pub telegram_bot_token: String,
    pub telegram_chat_id: String,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}