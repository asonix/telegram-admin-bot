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

use futures::future::Future;
use telebot::bot::RcBot;
use telebot::objects::{Integer, Message, Update, User};
use telebot::functions::*;

fn user_string(user: &User) -> String {
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

    user_str
}

fn kind(bot: &RcBot, msg: &Message, chat_id: Integer) -> &'static str {
    if let Some(ref audio) = msg.audio {
        bot.inner.handle.spawn(
            bot.audio(chat_id)
                .audio(audio.file_id.clone())
                .send()
                .map(|_| ())
                .map_err(|e| error!("Error: {:?}", e)),
        );
        "audio"
    } else if let Some(ref document) = msg.document {
        bot.inner.handle.spawn(
            bot.document(chat_id)
                .document(document.file_id.clone())
                .send()
                .map(|_| ())
                .map_err(|e| error!("Error: {:?}", e)),
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
                .map_err(|e| error!("Error: {:?}", e)),
        );
        "photo"
    } else if let Some(ref sticker) = msg.sticker {
        bot.inner.handle.spawn(
            bot.sticker(chat_id)
                .sticker(sticker.file_id.clone())
                .send()
                .map(|_| ())
                .map_err(|e| error!("Error: {:?}", e)),
        );
        "sticker"
    } else if let Some(ref video) = msg.video {
        bot.inner.handle.spawn(
            bot.video(chat_id)
                .video(video.file_id.clone())
                .send()
                .map(|_| ())
                .map_err(|e| error!("Error: {:?}", e)),
        );
        "video"
    } else if let Some(ref voice) = msg.voice {
        bot.inner.handle.spawn(
            bot.voice(chat_id)
                .voice(voice.file_id.clone())
                .send()
                .map(|_| ())
                .map_err(|e| error!("Error: {:?}", e)),
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
                .map_err(|e| error!("Error: {:?}", e)),
        );
        "contact"
    } else if let Some(ref location) = msg.location {
        bot.inner.handle.spawn(
            bot.location(chat_id, location.longitude, location.latitude)
                .send()
                .map(|_| ())
                .map_err(|e| error!("Error: {:?}", e)),
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
                .map_err(|e| error!("Error: {:?}", e)),
        );
        "venue"
    } else {
        ""
    }
}

pub fn forward(bot: RcBot, update: Update, chat_id: Integer) -> Option<(RcBot, Update)> {
    if let Some(ref msg) = update.message {
        if msg.chat.kind == "private" {
            if msg.forward_from.is_some() {
                let mut text = Vec::new();
                text.push(String::from("New Report"));
                text.push(String::new());

                if let Some(ref user) = msg.forward_from {
                    text.push(user_string(user));
                }

                let kind = kind(&bot, msg, chat_id);
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
                    .map_err(|e| error!("Error: {:?}", e))
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
                        .map_err(|e| error!("Error: {:?}", e)),
                );
            }
        }
    }

    Some((bot, update))
}
