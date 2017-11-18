use telebot::bot::RcBot;
use telebot::objects::Message;
use telebot::error::Error as BotError;
use futures::future::Future;

use telebot::functions::*;

pub fn admins<'a>(
    tup: (RcBot, Message),
) -> impl Future<Item = (RcBot, Message), Error = BotError> + 'a {
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
