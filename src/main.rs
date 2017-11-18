extern crate telebot;
extern crate tokio_core;
extern crate futures;
extern crate dotenv;
extern crate admin_bot;

use telebot::bot::RcBot;
use tokio_core::reactor::Core;
use futures::IntoFuture;
use futures::stream::Stream;

use admin_bot::Config;
use admin_bot::commands::*;

fn main() {
    let config = Config::new();

    let mut lp = Core::new().unwrap();
    let bot = RcBot::new(lp.handle(), config.token()).update_interval(100);

    let chat_id = config.admin_chat_id();

    bot.register(bot.new_cmd("/start").and_then(start));
    bot.register(bot.new_cmd("/admins").and_then(admins));
    bot.register(bot.new_cmd("/relay").and_then(move |(bot, msg)| {
        relay(bot, msg, chat_id)
    }));

    let stream = bot.get_stream().filter_map(|(bot, update)| {
        forward(bot, update, chat_id)
    });

    lp.run(stream.for_each(|_| Ok(())).into_future()).unwrap();
}
