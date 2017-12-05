# AdminBot
AdminBot was created to assist in managing a series of related groupchats on Telegram.

[Crates.io](https://crates.io/crates/admin_bot)

### Information
1. If you wish to say something to the admins in chat, you can type `/relay` with thatever it is you want to tell the admins and the bot will forward what you said to a designated admin chat.
2. If you want to be anonymous, you can DM the bot and whatever you send will be anonymously forwarded to a designated admin chat.
3. If you want to point out something specific from a chat, you can forward any messages you have an issue with directly to the bot and the forwarded messages along with who originally posted those messages will be anonymously sent to a designated admin chat.
4. If you just want to @ the admins of the current chat you're in you can do so by typing `/admins`

### Using the bot

First, make sure you talk to @BotFather to get a bot token. Add your bot to the admin chat, and then send a message.

Go to `https://api.telegram.org/bot<YourBOTToken>/getUpdates` to view the current updates for your bot, and get the chat ID. It should be a negative number.

#### From Release Binary
If there is a [release for your operating system and architecture](https://github.com/asonix/telegram-admin-bot/releases), you can use the following command to run it:
```bash
TELEGRAM_BOT_TOKEN="your token" \
ADMIN_CHAT_ID="your admin chat" \
./path/to/the/binary
```

#### From crates.io
This bot requires Rust Nightly to compile.

The following command will download telecord, compile it, and put the binary in `~/.cargo/bin`
```bash
cargo install telecord
```

#### From Source
This bot requires Rust Nightly to compile.

copy .env.sample to .env and set the required values.
```
TELEGRAM_BOT_TOKEN="123456789:ABCDEFGHIJK_LMNOPQRSTUVWXYZ123456"
ADMIN_CHAT_ID="-123456789"
```

Run `cargo run` from the project directory to run the project.

If you want to install this binary directly with `cargo install`, make sure the `TELEGRAM_BOT_TOKEN` and `ADMIN_CHAT_ID` environment variables are set.

#### As a system process with SystemD
After you have the bot compiled, copy the binary wherever you want it and make a systemd unit file based on the following template.
```
[Unit]
Description=A bot to help with adminning Coconuts
After=network.target

[Service]
Type=simple
User=your-admin_bot-user
Group=your-admin_bot-group
Environment="TELEGRAM_BOT_TOKEN=123456789:ABCDEFGHIJK_LMNOPQRSTUVWXYZ123456"
Environment="ADMIN_CHAT_ID=-123456789"
Environment="RUST_LOG=admin_bot=info"
ExecStart=/path/to/admin_bot/binary
TimeoutSec=90
Restart=always

[Install]
WantedBy=default.target
```

### License

AdminBot is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

AdminBot is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of AdminBot

You should have received a copy of the GNU General Public License along with AdminBot If not, see http://www.gnu.org/licenses/.
