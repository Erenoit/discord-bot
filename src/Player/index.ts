// Discord API
import { Collection, Message } from "discord.js";
import Voice, { AudioPlayer, AudioResource, DiscordGatewayAdapterCreator, VoiceConnection } from "@discordjs/voice";

// Node.js
import { readdirSync } from "fs";
import path            from "path";
import { Readable }    from "stream";

// YouTube API
import yt_open     from "ytdl-core";
import yt_search   from "ytsr";
import yt_playlist from "ytpl";

// Interaces
import { PlayerEvent, Song } from "../Interfaces";

class Player {
  public  events:      Collection<string, PlayerEvent> = new Collection()
  private songQueue:   Array<Song> = [];
  private now_playing: Song;
  }

  public init_events() {
    console.log("----- Generating Player Events -----");
    const event_path = path.join(__dirname, "..", "PlayerEvents");
    readdirSync(event_path).forEach(async (file) => {
      const { event } = await import(`${event_path}/${file}`);
      this.events.set(event.name, event);

      console.log(event);

//      this.player.on(event.name, event.run.bind(null));
    });
  }

  public joinVC(message: Message, args?: string[]) {
    const channelID = message.member?.voice?.channel?.id;
    const guildID   = message.guild?.id;
    const adapter   = message.guild?.voiceAdapterCreator as unknown as DiscordGatewayAdapterCreator;

    if(channelID && guildID && adapter) {
      this.connection = Voice.joinVoiceChannel({
        channelId: channelID,
        guildId: guildID,
        adapterCreator: adapter,
        selfDeaf: true
      });
    }
    else {
      message.reply("Failed to join to voice channel. (Posibly you are not in a joice channel.)");
    }
  }

  public async play(message: Message, args: string[]) {
  }

  public async stop(message?: Message) {
  }

  public async skip(message: Message) {
  }

  public async queue(message: Message) {
		if (!queue) { message.channel.send('Nothing playing right now!'); } 
    else {
  }
}

export default Player;
