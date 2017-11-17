#![feature(conservative_impl_trait)]

extern crate telebot;
extern crate tokio_core;
extern crate futures;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use telebot::bot;
use telebot::objects;
use telebot::error::Error as BotError;
use tokio_core::reactor::Core;
use futures::Future;
use futures::IntoFuture;
use futures::stream::Stream;

use telebot::functions::*;

type MsgTuple = (bot::RcBot, objects::Message);

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

fn start<'a>(tup: MsgTuple) -> impl Future<Item = MsgTuple, Error = BotError> + 'a {
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

fn relay<'a>(
    bot: bot::RcBot,
    msg: objects::Message,
    chat_id: objects::Integer,
) -> impl Future<Item = MsgTuple, Error = BotError> + 'a {
    if msg.chat.kind != "private" {
        let mut text = msg.text.unwrap().clone();

        if text.is_empty() {
            text = "<empty>".into();
        }

        bot.message(chat_id, text).send()
    } else {
        bot.message(
            msg.chat.id,
            "Please talk directly to the bot, you don't need to use commands".into(),
        ).send()
    }
}

fn forward<'a>(
    bot: bot::RcBot,
    update: objects::Update,
    chat_id: objects::Integer,
) -> Option<(bot::RcBot, objects::Update)> {
    if let Some(ref msg) = update.message {
        if msg.chat.kind == "private" {
            if msg.forward_from.is_some() {
                let mut text = Vec::new();
                text.push(String::from("New Report"));
                text.push(String::new());

                if let Some(ref user) = msg.forward_from {
                    let mut user_str = String::from("User: ");
                    user_str.push_str(&user.first_name);

                    if let Some(ref last_name) = user.last_name {
                        user_str.push(' ');
                        user_str.push_str(last_name);
                    }

                    if let Some(ref username) = user.username {
                        user_str.push_str(", @");
                        user_str.push_str(username);
                    }

                    text.push(user_str);
                }

                let kind = if let Some(ref audio) = msg.audio {
                    bot.inner.handle.spawn(
                        bot.audio(chat_id)
                            .audio(audio.file_id.clone())
                            .send()
                            .map(|_| ())
                            .map_err(|_| ()),
                    );
                    "audio"
                } else if let Some(ref document) = msg.document {
                    bot.inner.handle.spawn(
                        bot.document(chat_id)
                            .document(document.file_id.clone())
                            .send()
                            .map(|_| ())
                            .map_err(|_| ()),
                    );
                    "document"
                } else if msg.game.is_some() {
                    "game"
                } else if let Some(ref photo) = msg.photo {
                    bot.inner.handle.spawn(
                        bot.photo(chat_id)
                            .photo(photo[0].file_id.clone())
                            .send()
                            .map(|_| ())
                            .map_err(|_| ()),
                    );
                    "photo"
                } else if let Some(ref sticker) = msg.sticker {
                    bot.inner.handle.spawn(
                        bot.sticker(chat_id)
                            .sticker(sticker.file_id.clone())
                            .send()
                            .map(|_| ())
                            .map_err(|_| ()),
                    );
                    "sticker"
                } else if let Some(ref video) = msg.video {
                    bot.inner.handle.spawn(
                        bot.video(chat_id)
                            .video(video.file_id.clone())
                            .send()
                            .map(|_| ())
                            .map_err(|_| ()),
                    );
                    "video"
                } else if let Some(ref voice) = msg.voice {
                    bot.inner.handle.spawn(
                        bot.voice(chat_id)
                            .voice(voice.file_id.clone())
                            .send()
                            .map(|_| ())
                            .map_err(|_| ()),
                    );
                    "voice"
                } else if let Some(ref contact) = msg.contact {
                    bot.inner.handle.spawn(
                        bot.contact(
                            chat_id,
                            contact.phone_number.clone(),
                            contact.first_name.clone(),
                        ).send()
                            .map(|_| ())
                            .map_err(|_| ()),
                    );
                    "contact"
                } else if let Some(ref location) = msg.location {
                    bot.inner.handle.spawn(
                        bot.location(chat_id, location.longitude, location.latitude)
                            .send()
                            .map(|_| ())
                            .map_err(|_| ()),
                    );
                    "location"
                } else if let Some(ref venue) = msg.venue {
                    let v = bot.venue(
                        chat_id,
                        venue.location.longitude,
                        venue.location.latitude,
                        venue.title.clone(),
                        venue.address.clone(),
                    );

                    bot.inner.handle.spawn(
                        if let Some(ref foursquare_id) = venue.foursquare_id {
                            v.foursquare_id(foursquare_id.clone()).send()
                        } else {
                            v.send()
                        }.map(|_| ())
                            .map_err(|_| ()),
                    );
                    "venue"
                } else {
                    ""
                };

                if !kind.is_empty() {
                    let mut knd = String::from("Kind: ");
                    knd.push_str(kind);
                    text.push(knd);
                }

                if let Some(ref caption) = msg.caption {
                    let mut cptn = String::from("Caption: ");
                    cptn.push_str(caption);
                    text.push(cptn);
                }

                if let Some(ref msg_text) = msg.text {
                    let mut content = String::from("Content: ");
                    content.push_str(msg_text);
                    text.push(content);
                }

                bot.inner.handle.spawn(
                    bot.message(chat_id, text.join("\n"))
                        .send()
                        .join(bot.message(msg.chat.id, String::from("Report sent\n\nIf you would like to provide more information, please send it in this chat")).send())
                        .map(|_| ())
                        .map_err(|_| ()),
                );
                return None;
            } else {
                let mut text = Vec::new();
                text.push(String::from("Anonymous Submission"));
                text.push(String::new());

                if let Some(ref msg_text) = msg.text {
                    text.push(String::from(msg_text.as_ref()));
                }

                bot.inner.handle.spawn(
                    bot.message(chat_id, text.join("\n"))
                        .send()
                        .join(
                            bot.message(msg.chat.id, String::from("Message sent"))
                                .send(),
                        )
                        .map(|_| ())
                        .map_err(|_| ()),
                );
            }
        }
    }

    Some((bot, update))
}

fn admins<'a>(tup: MsgTuple) -> impl Future<Item = MsgTuple, Error = BotError> + 'a {
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

fn main() {
    let config = Config::new();

    let mut lp = Core::new().unwrap();
    let bot = bot::RcBot::new(lp.handle(), &config.token).update_interval(100);

    let chat_id = config.admin_chat_id;

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
