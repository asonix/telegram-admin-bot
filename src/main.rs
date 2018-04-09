// This file is part of AdminBot

// AdminBot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AdminBot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AdminBot  If not, see <http://www.gnu.org/licenses/>.

extern crate admin_bot;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate telebot;
extern crate tokio_core;

use std::collections::HashMap;
use std::fs::File;

use telebot::bot::RcBot;
use tokio_core::reactor::Core;
use futures::stream::Stream;
use futures::future::Future;
use futures::sync::mpsc::channel;

use telebot::functions::*;

use admin_bot::STATES_JSON;
use admin_bot::Config;
use admin_bot::commands::*;

fn init_bot(bot: &RcBot) {
    bot.inner.handle.spawn(
        bot.get_me()
            .send()
            .map_err(|e| println!("Error: {:?}", e))
            .and_then(|(bot, user)| {
                let pairs = bot.inner
                    .handlers
                    .borrow()
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect::<Vec<_>>();

                let username = if let Some(username) = user.username {
                    username
                } else {
                    return Err(());
                };

                for (key, value) in pairs {
                    bot.inner
                        .handlers
                        .borrow_mut()
                        .insert(format!("{}@{}", key, username), value);
                }

                Ok(())
            }),
    );
}

fn main() {
    env_logger::init();
    info!("Starting bot");
    let config = Config::new();

    let mut lp = Core::new().unwrap();
    let bot = RcBot::new(lp.handle(), config.token()).update_interval(100);

    init_bot(&bot);

    let chat_id = config.admin_chat_id();

    bot.register(bot.new_cmd("/start").and_then(start));
    bot.register(bot.new_cmd("/admins").and_then(admins));
    bot.register(
        bot.new_cmd("/relay")
            .and_then(move |(bot, msg)| relay(&bot, &msg, chat_id)),
    );
    bot.register(
        bot.new_cmd("/health")
            .and_then(move |(bot, msg)| health_check(&bot, &msg)),
    );

    bot.inner.handle.spawn(
        bot.message(chat_id, "Bot Started".into())
            .send()
            .map(|_| ())
            .map_err(|e| error!("Error: {:?}", e)),
    );

    let (tx, rx) = channel::<Active>(100);

    let state = File::open(STATES_JSON)
        .map_err(|_| ())
        .and_then(|file| serde_json::from_reader(file).map_err(|_| ()))
        .unwrap_or(HashMap::new());

    bot.inner.handle.spawn(timeout_stream(
        TimeoutState::from_hashmap(state, bot.clone()),
        bot.clone(),
        rx,
    ));

    let stream = bot.get_stream()
        .filter_map(|(bot, update)| forward(bot, update, chat_id))
        .and_then(move |(bot, update)| timeout(bot, update, tx.clone()))
        .map_err(|e| error!("Error: {:?}", e))
        .for_each(|_| Ok(()));

    lp.run(stream).unwrap();
}
