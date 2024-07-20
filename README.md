# The Bot
**The Bot** is a basic music bot for [Discord](https://discord.com/) written in [Rust](https://www.rust-lang.org/). If you want to use old, [TypeScript](https://www.typescriptlang.org/) implementation use [old-typescript](https://gitlab.com/Erenoit/discord-bot/-/tree/old-typescript) branch.

## Quick Start
1. Clone this repository:
```shell
$ git clone https://gitlab.com/Erenoit/discord-bot.git
```

2. Add environmental variables (or use a .env file in project root):

- **BOT_TOKEN:** Your bot's token,
- **BOT_PREFIX:** Prefix you want to use with commands,
- **BOT_SP_CLIENT_ID:** Spotify client ID,
- **BOT_SP_CLIENT_SECRET:** Spotify client secret

3. `cd` into repository and run the following commands:
```shell
$ cargo build --release
$ ./target/release/discord-bot
```

## Dependencies
- [Rust](https://www.rust-lang.org/)
- [Opus](https://opus-codec.org/)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp)
