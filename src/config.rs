use std::env;
use dotenv::dotenv;
use telebot::objects::Integer;

pub struct Config {
    token: String,
    admin_chat_id: Integer,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Config {
            token: env::var("TELEGRAM_BOT_TOKEN").expect(
                "Please set the TELEGRAM_BOT_TOKEN environment variable",
            ),
            admin_chat_id: env::var("ADMIN_CHAT_ID")
                .expect("Please set the ADMIN_CHAT_ID environment variable")
                .parse::<Integer>()
                .expect("Failed to parse chat id"),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn admin_chat_id(&self) -> Integer {
        self.admin_chat_id
    }
}
