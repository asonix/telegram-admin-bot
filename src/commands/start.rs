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
use telebot::objects::Message;
use failure::Error;
use futures::future::Future;

use telebot::functions::*;

pub fn start<'a>(
    tup: (RcBot, Message),
) -> impl Future<Item = (RcBot, Message), Error = Error> + 'a {
    let bot = tup.0;
    let msg = tup.1;

    let mut text = String::new();
    if msg.chat.kind == "private" {
        text.push_str("Talk to this bot to anonymously submit information to the admins\nForward messages to this bot to show the admins.");
    } else {
        text.push_str("Use /relay <your message> to send a message to the admins. Use /admins to tag all admins of the chat");
    }

    bot.message(msg.chat.id, text).send()
}
