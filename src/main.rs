#![feature(conservative_impl_trait)]

extern crate telebot;
extern crate tokio_core;
extern crate futures;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use telebot::bot;
use telebot::objects;
use tokio_core::reactor::Core;
use futures::stream::Stream;

use telebot::functions::*;

struct Config {
    token: String,
    admin_chat_id: objects::Integer,
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
                .parse::<objects::Integer>()
                .expect("Failed to parse chat id"),
        }
    }
}

fn main() {
    let config = Config::new();

    let mut lp = Core::new().unwrap();
    let bot = bot::RcBot::new(lp.handle(), &config.token).update_interval(100);

    let handle = bot.new_cmd("/relay").and_then(move |(bot, msg)| {
        let mut text = msg.text.unwrap().clone();

        if text.is_empty() {
            text = "<empty>".into();
        } else {
            println!("text: {}", &text);
        }

        bot.message(config.admin_chat_id, text).send()
    });

    bot.register(handle);
    bot.run(&mut lp).unwrap();
}
