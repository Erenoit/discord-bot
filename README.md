# The Bot
**The Bot** is a basic music bot for [Discord](https://discord.com/).

## Quick Start
1. Clone this repository:
    ```shell
    $ git clone https://gitlab.com/Erenoit/discord-bot.git
    ```
2. Create a file `config.json` in `src/` folder with,
    ```json
    {
      "token":  YOUR_BOT_TOKEN,
      "prefix": PREFERED_PREFİX
    }
    ```
3. `cd` into repository and run the following commands commands:
    ```shell
    $ yarn install
    $ yarn start
    ```

## Dependencies
- [Node.js](https://nodejs.org/)
- [FFmpeg](https://www.ffmpeg.org/download.html)
- [sodium](https://www.npmjs.com/package/sodium) or [libsodium-wrappers](https://www.npmjs.com/package/libsodium-wrappers)
- [Python](https://www.python.org/) *optional*
- [Yarn](https://yarnpkg.com/) *optional*
