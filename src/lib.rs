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

extern crate dotenv;
extern crate failure;
extern crate futures;
extern crate serde_json;
extern crate telebot;
extern crate tokio_timer;

#[macro_use]
extern crate log;

mod config;
pub mod commands;

pub use config::Config;

pub static STATES_JSON: &str = "states.json";
