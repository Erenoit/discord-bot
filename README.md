# The Bot
**The Bot** is a basic music bot for [Discord](https://discord.com/) written in [Rust](https://www.rust-lang.org/). This branch is not production ready at the moment. If you want to use this bot, [TypeScript](https://www.typescriptlang.org/) version is recomended. TypeScript version can be found in the [main](https://gitlab.com/Erenoit/discord-bot) branch.

## Quick Start
1. Clone this repository:
```shell
$ git clone https://gitlab.com/Erenoit/discord-bot.git
```

2. Add environmental variables (or use a .env file in project root):
- **TOKEN:** Your bot's token,
- **PREFIX:** Prefix you want to use with commands,

3. `cd` into repository and run the following commands:
```shell
$ cargo build --release
$ ./target/release/discord-bot
```

## Dependencies
- [Rust](https://www.rust-lang.org/)
- [FFmpeg](https://www.ffmpeg.org/download.html)
- [youtube-dl](https://youtube-dl.org/)
- [Opus](https://opus-codec.org/)
