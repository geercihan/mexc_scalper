use crate::config::Config;
use reqwest::Client;
use serde_json::json;

pub async fn send_telegram_message(config: &Config, message: &str) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        config.telegram_bot_token
    );

    let payload = json!({
        "chat_id": config.telegram_chat_id,
        "text": message
    });

    let client = Client::new();
    let res = client.post(&url).json(&payload).send().await?;

    if config.debug_mode {
        println!("🔔 Telegram status: {}", res.status());
    }

    Ok(())
}