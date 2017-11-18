use telebot::bot::RcBot;
use telebot::error::Error as BotError;
use telebot::objects::{Integer, Message};
use futures::future::Future;

use telebot::functions::*;

pub fn relay<'a>(
    bot: RcBot,
    msg: Message,
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
