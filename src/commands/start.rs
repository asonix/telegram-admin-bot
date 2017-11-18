use telebot::bot::RcBot;
use telebot::error::Error as BotError;
use telebot::objects::Message;
use futures::future::Future;

use telebot::functions::*;

pub fn start<'a>(
    tup: (RcBot, Message),
) -> impl Future<Item = (RcBot, Message), Error = BotError> + 'a {
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
