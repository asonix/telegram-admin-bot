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

use std::collections::HashMap;
use std::fs::File;
use std::time::{Duration, SystemTime};

use failure::Error;
use futures::{Future, Sink, Stream};
use futures::future::{result, Either};
use futures::sync::mpsc::{Receiver, Sender};
use futures::sync::oneshot::{self, channel};
use serde_json;
use telebot::bot::RcBot;
// use telebot::functions::*;
use telebot::objects::{Integer, Update};
use tokio_timer::Timer;

use STATES_JSON;

pub struct Active {
    chat_id: Integer,
    user_id: Integer,
    action: ActiveAction,
}

pub enum ActiveAction {
    Talked,
    Joined,
    Left,
}

pub struct EndWait;

fn warn_user(_bot: RcBot, user_id: Integer, chat_id: Integer) {
    // bot.inner.handle.spawn(bot.kick_chat_member(chat_id, user_id).send())
    println!(
        "Not warning user {} of impending ban from {}",
        user_id, chat_id
    );
}

fn kick_user(_bot: RcBot, user_id: Integer, chat_id: Integer) {
    println!(
        "Banning user {} from {} due to inactivity",
        user_id, chat_id
    );
    /*
    bot.inner.handle.spawn(
        bot.kick_chat_member(chat_id, user_id)
            .send()
            .map(|_| ())
            .map_err(|_| ()),
    )
    */
}

pub struct TimeoutState {
    active: SystemTime,
    senders: TimeoutSenders,
}

impl TimeoutState {
    pub fn from_tuple(
        (chat_id, user_id, time): (Integer, Integer, SystemTime),
        bot: RcBot,
    ) -> Option<Self> {
        let one_month = Duration::from_secs(60 * 60 * 24 * 30 * 1);
        let three_months = Duration::from_secs(60 * 60 * 24 * 30 * 3);
        let four_months = Duration::from_secs(60 * 60 * 24 * 30 * 4);
        let now = SystemTime::now();

        let bot_2 = bot.clone();

        if time + four_months < now {
            kick_user(bot, user_id, chat_id);
            None
        } else if time + three_months < now {
            // kick user after warning
            let (tx2, rx2) = channel::<EndWait>();

            let sleep_duration =
                one_month - now.duration_since(time).unwrap_or(Duration::from_secs(1));

            bot.inner.handle.clone().spawn(
                Timer::default()
                    .sleep(sleep_duration)
                    .select2(rx2)
                    .map_err(|_| ())
                    .and_then(move |_| {
                        kick_user(bot, user_id, chat_id);
                        Ok(())
                    })
                    .map_err(move |_| println!("Killed timers for {}", user_id)),
            );

            Some(TimeoutState {
                active: SystemTime::now(),
                senders: TimeoutSenders::Warned(tx2),
            })
        } else if time < now {
            // wait to deliver warning
            let (tx1, rx1) = channel::<EndWait>();
            let (tx2, rx2) = channel::<EndWait>();

            let sleep_duration =
                three_months - now.duration_since(time).unwrap_or(Duration::from_secs(1));

            bot.inner.handle.clone().spawn(
                Timer::default()
                    .sleep(sleep_duration)
                    .select2(rx1)
                    .map_err(|_| ())
                    .and_then(|either| match either {
                        Either::A(_) => Ok(()),
                        Either::B(_) => Err(()),
                    })
                    .and_then(move |_| {
                        warn_user(bot, user_id, chat_id);
                        // sleep case
                        Timer::default()
                            .sleep(one_month)
                            .select2(rx2)
                            .map_err(|_| ())
                    })
                    .and_then(move |_| {
                        kick_user(bot_2, user_id, chat_id);
                        Ok(())
                    })
                    .map_err(move |_| println!("Killed timers for {}", user_id)),
            );

            Some(TimeoutState {
                active: SystemTime::now(),
                senders: TimeoutSenders::Start(tx1, tx2),
            })
        } else {
            let (tx1, rx1) = channel::<EndWait>();
            let (tx2, rx2) = channel::<EndWait>();

            bot.inner.handle.clone().spawn(
                Timer::default()
                    .sleep(three_months)
                    .select2(rx1)
                    .map_err(|_| ())
                    .and_then(|either| match either {
                        Either::A(_) => Ok(()),
                        Either::B(_) => Err(()),
                    })
                    .and_then(move |_| {
                        warn_user(bot, user_id, chat_id);
                        // sleep case
                        Timer::default()
                            .sleep(one_month)
                            .select2(rx2)
                            .map_err(|_| ())
                    })
                    .and_then(move |_| {
                        kick_user(bot_2, user_id, chat_id);
                        Ok(())
                    })
                    .map_err(move |_| println!("Killed timers for {}", user_id)),
            );

            Some(TimeoutState {
                active: SystemTime::now(),
                senders: TimeoutSenders::Start(tx1, tx2),
            })
        }
    }

    pub fn from_hashmap(
        states: HashMap<Integer, HashMap<Integer, SystemTime>>,
        bot: RcBot,
    ) -> HashMap<Integer, HashMap<Integer, Self>> {
        states
            .into_iter()
            .map(move |(chat_id, user_map)| {
                (
                    chat_id,
                    user_map
                        .into_iter()
                        .fold(HashMap::new(), |mut inner_acc, (user_id, time)| {
                            TimeoutState::from_tuple((chat_id, user_id, time), bot.clone())
                                .map(|state| inner_acc.insert(user_id, state));

                            inner_acc
                        }),
                )
            })
            .collect()
    }

    pub fn to_hashmap(
        states: &HashMap<Integer, HashMap<Integer, Self>>,
    ) -> HashMap<Integer, HashMap<Integer, SystemTime>> {
        states
            .iter()
            .map(move |(chat_id, user_map)| {
                (
                    *chat_id,
                    user_map
                        .iter()
                        .map(|(user_id, state)| (*user_id, state.active.clone()))
                        .collect(),
                )
            })
            .collect()
    }
}

pub enum TimeoutSenders {
    Start(oneshot::Sender<EndWait>, oneshot::Sender<EndWait>),
    Warned(oneshot::Sender<EndWait>),
}

pub fn timeout_stream(
    state: HashMap<Integer, HashMap<Integer, TimeoutState>>,
    bot: RcBot,
    rx: Receiver<Active>,
) -> impl Future<Item = (), Error = ()> {
    rx.map_err(|_| ())
        .fold(state, move |mut acc, active| {
            let chat_id = active.chat_id;
            let user_id = active.user_id;
            let action = active.action;

            {
                let user_map = acc.entry(chat_id).or_insert(HashMap::new());

                match action {
                    ActiveAction::Talked | ActiveAction::Joined => {
                        match user_map.remove(&user_id) {
                            Some(state) => match state.senders {
                                TimeoutSenders::Start(tx1, tx2) => {
                                    let _ = tx1.send(EndWait);
                                    let _ = tx2.send(EndWait);
                                }
                                TimeoutSenders::Warned(tx2) => {
                                    let _ = tx2.send(EndWait);
                                }
                            },
                            None => (),
                        };

                        let (tx1, rx1) = channel::<EndWait>();
                        let (tx2, rx2) = channel::<EndWait>();

                        user_map.insert(
                            user_id,
                            TimeoutState {
                                active: SystemTime::now(),
                                senders: TimeoutSenders::Start(tx1, tx2),
                            },
                        );

                        let bot_2 = bot.clone();
                        let bot_3 = bot.clone();

                        bot.inner.handle.clone().spawn(
                            Timer::default()
                                .sleep(Duration::from_secs(60 * 60 * 24 * 30 * 3))
                                .select2(rx1)
                                .map_err(|_| ())
                                .and_then(|either| match either {
                                    Either::A(_) => Ok(()),
                                    Either::B(_) => Err(()),
                                })
                                .and_then(move |_| {
                                    warn_user(bot_2, user_id, chat_id);
                                    // sleep case
                                    Timer::default()
                                        .sleep(Duration::from_secs(60 * 60 * 24 * 30 * 1))
                                        .select2(rx2)
                                        .map_err(|_| ())
                                })
                                .and_then(move |_| {
                                    kick_user(bot_3, user_id, chat_id);
                                    // kick user
                                    Ok(())
                                })
                                .map_err(move |_| println!("Killed timers for {}", user_id)),
                        );
                    }
                    ActiveAction::Left => {
                        user_map.remove(&user_id);
                    }
                };
            }

            let res = File::create(STATES_JSON).map_err(|_| ()).and_then(|file| {
                serde_json::to_writer(file, &TimeoutState::to_hashmap(&acc)).map_err(|_| ())
            });

            if res.is_ok() {
                println!("wrote");
            }

            Ok(acc) as Result<_, ()>
        })
        .map(|_| ())
        .or_else(|_| Ok(()))
}

pub fn timeout(
    bot: RcBot,
    update: Update,
    tx: Sender<Active>,
) -> impl Future<Item = (RcBot, Update), Error = Error> {
    let msg = if let Some(ref msg) = update.message {
        if let Some(ref user) = msg.from {
            Ok((user.id, msg.chat.id, ActiveAction::Talked))
        } else if let Some(ref user) = msg.new_chat_member {
            Ok((user.id, msg.chat.id, ActiveAction::Joined))
        } else if let Some(ref user) = msg.left_chat_member {
            Ok((user.id, msg.chat.id, ActiveAction::Left))
        } else {
            Err(())
        }
    } else {
        Err(())
    };

    result(msg)
        .and_then(move |(user_id, chat_id, action)| {
            tx.send(Active {
                chat_id,
                user_id,
                action,
            }).map_err(|_| ())
        })
        .then(|_| Ok((bot, update)))
}
