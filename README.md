# The Bot
**The Bot** is a basic music bot for [Discord](https://discord.com/).

## Quick Start
1. Clone this repository:
    ```shell
    $ git clone https://gitlab.com/Erenoit/discord-bot.git
    ```
2. Create a file `config.json` in `src/` folder with (you can ignore the ones with `?`),
    ```json
    {
      "token": YOUR_BOT_TOKEN,
      "prefix": PREFERED_PREFİX,
      "yt_cookie"?: YOUR_YOUTUBE_COOKİE
      "spotify"?: {
        "client_id": YOUR_SPOTIFY_CLIENT_ID
        "client_secret": YOUR_SPOTIFY_CLIENT_SECRET
        "refresh_token": YOUR_SPOTİDY_REFRESH_TOKEN
      }
    }
    ```
3. `cd` into repository and run the following commands commands:
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
