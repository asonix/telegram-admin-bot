# AdminBot
AdminBot was created to assist in managing a series of related groupchats on Telegram.

### Information
1. If you wish to say something to the admins in chat, you can type `/relay` with thatever it is you want to tell the admins and the bot will forward what you said to a designated admin chat.
2. If you want to be anonymous, you can DM the bot and whatever you send will be anonymously forwarded to a designated admin chat.
3. If you want to point out something specific from a chat, you can forward any messages you have an issue with directly to the bot and the forwarded messages along with who originally posted those messages will be anonymously sent to a designated admin chat.
4. If you just want to @ the admins of the current chat you're in you can do so by typing `/admins`

### Using the bot
First, make sure you talk to @BotFather to get a bot token. Add your bot to the admin chat, and then send a message.

Go to `https://api.telegram.org/bot<YourBOTToken>/getUpdates` to view the current updates for your bot, and get the chat ID. It should be a negative number.

copy .env.sample to .env and set the required values.
```
TELEGRAM_BOT_TOKEN="123456789:ABCDEFGHIJK_LMNOPQRSTUVWXYZ123456"
ADMIN_CHAT_ID="-123456789"
```

Run `cargo run` from the project directory to run the project.

If you want to install this binary directly with `cargo install`, make sure the `TELEGRAM_BOT_TOKEN` and `ADMIN_CHAT_ID` environment variables are set.

Here is an example Systemd Unit file
```
[Unit]
Description=A bot to help with adminning Coconuts
After=network.target

[Service]
User=admins
Group=admins
Environment="TELEGRAM_BOT_TOKEN=123456789:ABCDEFGHIJK_LMNOPQRSTUVWXYZ123456"
Environment="ADMIN_CHAT_ID=-123456789"
ExecStart=/home/admins/.cargo/bin/admin_bot

[Install]
WantedBy=default.target
```

### License

AdminBot is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

AdminBot is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of AdminBot

You should have received a copy of the GNU General Public License along with AdminBot If not, see http://www.gnu.org/licenses/.
