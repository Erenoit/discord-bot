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

  public async shuffle(message: Message) {
    // The modern version of the Fisher–Yates shuffle algorithm
    for(let i = this.songQueue.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      const tmp = this.songQueue[i];
      this.songQueue[i] = this.songQueue[j];
      this.songQueue[j] = tmp;
    }

    message.reply("Queue is shuffled. (You cannot undo shuffleing.)");
  }

  public async queue(message: Message) {
    let reply_message = `Currently playing \`${this.now_playing.name}\` [${this.now_playing.length}], requested by **${this.now_playing.user_name}**\n`;
    const queue_length = this.songQueue.length;

    if (queue_length <= 10) {
      for (let i = 0; i < queue_length; i++) {
        reply_message += `**${i}** \`${this.songQueue[i].name}\` [${this.songQueue[i].length}]\n`;
      }
    }
    else {
      for (let i = 0; i <= 10; i++) {
        reply_message += `**${i}** \`${this.songQueue[i].name}\` [${this.songQueue[i].length}]\n`;
      }
      reply_message += `And ${queue_length - 10} more...`;
    }

    message.reply(reply_message);
  }

  private changeStream(value: Readable) {
    this.stream = value;
    this.resource = Voice.createAudioResource(this.stream);
    this.player.play(this.resource);
    this.connection.subscribe(this.player);
  }

  private start() {
    this.now_playing = this.songQueue.shift() as Song;

    this.changeStream(yt_open(this.now_playing.url, this.yt_options));

    this.player.on(Voice.AudioPlayerStatus.Idle, () => {
      if (this.songQueue.length > 0) {
        this.start();
      }
      else {
        this.stop();
      }
    });
  }
}

export default Player;
