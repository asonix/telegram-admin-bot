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
use telebot::error::Error as BotError;
use futures::future::Future;

use telebot::functions::*;

pub fn health_check<'a>(
    tup: (RcBot, Message),
) -> impl Future<Item = (RcBot, Message), Error = BotError> + 'a {
    tup.0.message(tup.1.chat.id, "Running".into()).send()
}
