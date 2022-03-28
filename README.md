# The Bot
**The Bot** is a basic music bot for [Discord](https://discord.com/).

## Quick Start
1. Clone this repository:
    ```shell
    $ git clone https://gitlab.com/Erenoit/discord-bot.git
    ```
2. Add evironmental variables (or use a .env file in project root) with (you can ignore the ones with `?`),
    ```.env
    TOKEN=YOUR_BOT_TOKEN,
    PREFIX=PREFERED_PREFIX,
    YT_COOKIE?=YOUR_YOUTUBE_COOKIE
    SP_CLIENT_ID?=YOUR_SPOTIFY_CLIENT_ID
    SP_CLIENT_SECRET?=YOUR_SPOTIFY_CLIENT_SECRET
    SP_REFRESH_TOKEN?=YOUR_SPOTIFY_REFRESH_TOKEN
    ```
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
