# TwitchBot
This is a twitch bot for CS:GO KZ.

It talks to different APIs and its own Database to provide useful functionality that you would
otherwise only get ingame or from a website like [KZ:GO](https://kzgo.eu). This functionality
mostly revolves around chat commands, including:
- `!pb`
- `!wr`
- `!recent`
- `!player`

and many more! For a full list check out the [Wiki](https://github.com/Schnose/TwitchBot/wiki).
I am running a public instance that you can invite to your channel by visiting [the account's
Twitch Channel](https://twitch.tv/SchnoseBot) and typing `!join` into the chat!

If you wish to run your own instance, read the following section.

## Setup

If you want to run your own instance of the bot, you can follow these steps:

1. Install dependencies:
  - [rustup](https://rustup.rs/)
  - [docker-compose](https://github.com/docker/compose)
  - [just](https://github.com/casey/just)

2. Clone this repo

```sh
git clone https://github.com/Schnose/TwitchBot.git
```

3. Create an account at https://www.shuttle.rs

4. Copy the `Secrets.example.toml` to `Secrets.dev.toml` (for running locally) and to `Secrets.toml`
   (for deploying) and modify the values according to your needs.

5. Run the project locally:

```sh
just dev
```

