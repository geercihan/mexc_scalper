mod config;
mod telegram;
mod executor;

use crate::config::load_config;
use crate::telegram::send_telegram_message;
use crate::executor::run_bot;

#[tokio::main]
async fn main() {
    let config = load_config("config.toml").expect("❌ فشل في قراءة ملف الإعدادات");
    println!("🧪 Simulation mode: {}", config.simulation);

    if config.telegram_enabled {
        let _ = send_telegram_message(&config, "✅ Mexc Scalper (Rust) جاهز للعمل.").await;
    }

    run_bot(config).await;
}