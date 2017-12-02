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

use telebot::bot::RcBot;
use telebot::error::Error as BotError;
use telebot::objects::{Integer, Message};
use futures::future::Future;

use telebot::functions::*;

pub fn relay<'a>(
    bot: &RcBot,
    msg: &Message,
    chat_id: Integer,
) -> impl Future<Item = (RcBot, Message), Error = BotError> + 'a {
    if msg.chat.kind != "private" {
        let text = match msg.text {
            Some(ref text) => text.clone(),
            None => String::new(),
        };

        bot.message(chat_id, text).send()
    } else {
        bot.message(
            msg.chat.id,
            "Please talk directly to the bot, you don't need to use commands".into(),
        ).send()
    }
}
