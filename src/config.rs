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

use std::env;
use dotenv::dotenv;
use telebot::objects::Integer;

pub struct Config {
    token: String,
    admin_chat_id: Integer,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn admin_chat_id(&self) -> Integer {
        self.admin_chat_id
    }
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();

        Config {
            token: env::var("TELEGRAM_BOT_TOKEN").expect(
                "Please set the TELEGRAM_BOT_TOKEN environment variable",
            ),
            admin_chat_id: env::var("ADMIN_CHAT_ID")
                .expect("Please set the ADMIN_CHAT_ID environment variable")
                .parse::<Integer>()
                .expect("Failed to parse chat id"),
        }
    }
}
