// Discord API
import { Collection, Message } from "discord.js";
import Voice, { AudioPlayer, AudioResource, DiscordGatewayAdapterCreator, NoSubscriberBehavior, VoiceConnection } from "@discordjs/voice";

// Node.js
import { readdirSync } from "fs";
import path            from "path";

// YouTube API
import playdl, { YouTubeStream } from "play-dl";

// Interaces
import { PlayerEvent, Song, StreamOptions } from "../Interfaces";

class Player {
  public  events:      Collection<string, PlayerEvent> = new Collection()
  private songQueue:   Array<Song> = [];
  private now_playing: Song;

  private player:      AudioPlayer = Voice.createAudioPlayer({ behaviors: { noSubscriber: NoSubscriberBehavior.Play }});
  private resource:    AudioResource;
  private connection:  VoiceConnection;
  private stream:      YouTubeStream;

  private stream_options: StreamOptions = {
    discordPlayerCompatibility: true
  };

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
    if (!this.connection) {
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
    else {
      this.connection.rejoin();
    }
  }

  public async play(message: Message, args: string[]) {
    this.joinVC(message);

    const argument  = args.join(" ");
    const user_name = message.member?.nickname;

    if (argument.search("http") === -1) {
      console.log("SEARCH");
      const raw_resoults = await playdl.search(argument, { limit: 1 });

      // TODO
      /*
      const resoults: Array<Song> = [];

      raw_resoults.items.map((raw_song) => {
        if (raw_song.type === "video") {
          const song: Song = {
            name: raw_song.title,
            url: raw_song.url,
            length: raw_song.duration,
            user_name: user_name
          }
          resoults.push(song);
        }
      });

      let reply_message = `Please select a song from following list with \`1-${resoults.length}\` \n`
      resoults.map((song, index) => {
        reply_message += `**${index + 1}:** ${song.name} \`${song.length}\`\n`;
      });
      reply_message += "**PLESAE WRITE 'none' FOR CANCEL**";

      message.reply(reply_message);

      //
      //
      //
      //
      //
      //
      */

      const song: Song = {
        name: raw_resoults[0].title as string,
        url: raw_resoults[0].url,
        length: raw_resoults[0].durationRaw,
        user_name: user_name
      }

      this.songQueue.push(song);

      message.reply(`${song.name} has been added to the queue.`);
    }
    else if (argument.search("list=") === -1) {
      console.log("URL");
      const raw_resoults = await playdl.video_info(argument);

      const song: Song = {
        name: raw_resoults.video_details.title as string,
        url: argument,
        length: raw_resoults.video_details.durationRaw,
        user_name: user_name
      }

      this.songQueue.push(song);

      message.reply(`${song.name} has been added to the queue.`);
    }
    else {
      console.log("PLAYLIST");
      const raw_resoults = (await playdl.playlist_info(argument, { incomplete: true })).toJSON();

      if (raw_resoults.videos) {
        raw_resoults.videos.map((raw_song) =>{
          const song: Song = {
            name: raw_song.title as string,
            url: raw_song.url,
            length: raw_song.durationRaw,
            user_name: user_name
          }
          this.songQueue.push(song);
        });

        message.reply(`**${raw_resoults.videos.length}** songs added to queue.`);
      }
      else {
        message.reply("Error happened while looking to playlist.");
      }
    }

    if (!this.now_playing) {
      this.start();
    }
  }

  public async stop(message?: Message) {
    this.connection.disconnect();

    this.now_playing = undefined as unknown as Song;
    this.songQueue = [];

    if (message) {
      message.reply("Goodbye. :sob: ");
    }
  }

  public async skip(message: Message) {
    message.reply(`\`${this.now_playing.name}\` is skipped`);

    this.start();
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

  private async changeStream(url: string) {
    this.stream = await playdl.stream(url, this.stream_options);
    this.resource = Voice.createAudioResource(this.stream.stream, { inputType: this.stream.type });
    this.player.play(this.resource);
    this.connection.subscribe(this.player);
  }

  private start() {
    this.now_playing = this.songQueue.shift() as Song;

    this.changeStream(this.now_playing.url);

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
