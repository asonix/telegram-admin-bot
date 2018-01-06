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

pub fn admins<'a>(
    tup: (RcBot, Message),
) -> impl Future<Item = (RcBot, Message), Error = Error> + 'a {
    let bot = tup.0;
    let msg = tup.1;
    let chat_id = msg.chat.id;

    bot.unban_chat_administrators(chat_id).send().and_then(
        move |(bot, members)| {
            let usernames: Vec<_> = members
                .iter()
                .map(|member| if let Some(ref username) = member.user.username {
                    let mut uname = String::from("@");
                    uname.push_str(username.as_ref());
                    uname
                } else {
                    String::from(member.user.first_name.as_ref())
                })
                .collect();

            let text = usernames.join(" ");

            bot.message(chat_id, text).send()
        },
    )
}
