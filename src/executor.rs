use crate::config::Config;
use crate::telegram::send_telegram_message;
use std::fs;
use std::collections::HashSet;
use tokio::time::{sleep, Duration};
use chrono::Utc;

pub async fn run_bot(config: Config) {
    let tokens = load_tokens();
    let mut executed: HashSet<String> = HashSet::new();

    for token in tokens {
        if executed.contains(&token) {
            continue;
        }

        if config.simulation {
            let msg = format!("📤 Simulated BUY and SELL executed for {token} @ {:?}", Utc::now());
            if config.telegram_enabled {
                let _ = send_telegram_message(&config, &msg).await;
            } else {
                println!("{}", msg);
            }
        }

        executed.insert(token);
        sleep(Duration::from_secs_f32(config.check_interval)).await;
    }
}

fn load_tokens() -> Vec<String> {
    let content = fs::read_to_string("tokens.txt").unwrap_or_default();
    content
        .lines()
        .map(|line| line.trim().to_uppercase())
        .filter(|line| !line.is_empty())
        .collect()
}