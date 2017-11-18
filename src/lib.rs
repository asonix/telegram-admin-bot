#![feature(conservative_impl_trait)]

extern crate telebot;
extern crate dotenv;
extern crate futures;

mod config;
pub mod commands;

pub use config::Config;
