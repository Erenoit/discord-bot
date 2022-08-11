# The Bot
**The Bot** is a basic music bot for [Discord](https://discord.com/).

## Quick Start
1. Clone this repository:
```shell
$ git clone https://gitlab.com/Erenoit/discord-bot.git
```

2. Add environmental variables (or use a .env file in project root):
- **TOKEN:** Your bot's token,
- **PREFIX:** Prefix you want to use with commands,
- **YT_COOKIE:** YouTube cookie *optional*
- **SP_CLIENT_ID:** Spotify client ID *optional*
- **SP_CLIENT_SECRET:** Spotify client secret *optional*
- **SP_REFRESH_TOKEN:** Spotify refresh token *optional*

**More information:** [playdl instructions](https://github.com/play-dl/play-dl/tree/main/instructions).

3. `cd` into repository and run the following commands:
```shell
$ yarn install
$ tsc
$ node ./compiled/index.js
```

## Dependencies
- [Node.js](https://nodejs.org/)
- [FFmpeg](https://www.ffmpeg.org/download.html)
- [libsodium-wrappers](https://www.npmjs.com/package/libsodium-wrappers)
- [Yarn](https://yarnpkg.com/) *optional*
- [Visual Studio Build Tools 2022 with "Desktop development with C++"](https://visualstudio.microsoft.com/downloads/) *only for Windows*
