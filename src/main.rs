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

extern crate telebot;
extern crate tokio_core;
extern crate futures;
extern crate dotenv;
extern crate admin_bot;

use telebot::bot::RcBot;
use tokio_core::reactor::Core;
use futures::IntoFuture;
use futures::stream::Stream;
use futures::future::Future;

use telebot::functions::*;

use admin_bot::Config;
use admin_bot::commands::*;

fn main() {
    println!("Starting bot");
    let config = Config::new();

    let mut lp = Core::new().unwrap();
    let bot = RcBot::new(lp.handle(), config.token()).update_interval(100);

    let chat_id = config.admin_chat_id();

    bot.register(bot.new_cmd("/start").and_then(start));
    bot.register(bot.new_cmd("/admins").and_then(admins));
    bot.register(bot.new_cmd("/relay").and_then(move |(bot, msg)| {
        relay(bot, msg, chat_id)
    }));
    bot.register(bot.new_cmd("/health").and_then(health_check));

    let stream = bot.get_stream().filter_map(|(bot, update)| {
        forward(bot, update, chat_id)
    });

    bot.inner.handle.spawn(
        bot.message(chat_id, "Bot Started".into())
            .send()
            .map(|_| ())
            .map_err(|_| ()),
    );

    lp.run(stream.for_each(|_| Ok(())).into_future()).unwrap();
}
