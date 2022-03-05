// Discord API
import { Collection, Message } from "discord.js";
import Voice, { AudioPlayer, AudioResource, DiscordGatewayAdapterCreator, NoSubscriberBehavior, VoiceConnection } from "@discordjs/voice";

// YouTube API
import playdl, { SpotifyAlbum, SpotifyPlaylist, YouTubeStream } from "play-dl";

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
    language: "en-US",
    quality: 2,
    discordPlayerCompatibility: true
  };

  constructor() {
    this.player.on("error", (err) => {
      console.log(err);
      this.start();
    });

    this.player.on(Voice.AudioPlayerStatus.Idle, () => {
        this.start();
    });
  }

  public setYTCookie(cookie: string) {
    playdl.setToken({
      youtube: {
          cookie: cookie
      }
    });
  }

  public async setSPToken(id: string, secret: string, token: string) {
    await playdl.setToken({
      spotify: {
        client_id: id,
        client_secret: secret,
        refresh_token: token,
        market: 'US'
      }
    });
  }

  public joinVC(message: Message, args?: string[]) {
    if (!this.connection) {
      console.log("FIRST TIME JOIN");
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
        message.reply("Failed to join to voice channel. (Posibly you are not in a voice channel.)");
      }
    }
    else {
      console.log("RECONNECTING");
      const sonnection = this.connection.rejoin();
      console.log(sonnection);
    }
  }

  public async play(message: Message, args: string[]) {
    this.joinVC(message);

    const argument  = args.join(" ");
    const user_name = message.member?.nickname;

    if (argument.search("http") === -1) {
      console.log("SEARCH");
      const raw_resoults = await playdl.search(argument, { limit: 1 }).catch( err => console.log(err) );

      if (raw_resoults && raw_resoults.length > 0) {
        const song: Song = {
          name: raw_resoults[0].title as string,
          url: raw_resoults[0].url,
          length: raw_resoults[0].durationRaw,
          user_name: user_name
        }
  
        this.songQueue.push(song);
  
        message.channel.send(`${song.name} has been added to the queue.`);
      }
      else {
        message.reply("Requested song could not be found. Try to search with different key words.");
      }
    }
    else if(argument.search("spotify") !== -1) {
      if (playdl.is_expired()) {
        await playdl.refreshToken();
      }

      const raw_resoults = await playdl.spotify(argument).catch( err => console.log(err) );
      console.log(raw_resoults);

      if (raw_resoults) {
        if(raw_resoults.type === "track") {
          const yt_resoult = await playdl.search(raw_resoults.name + " lyrics", { limit: 1 }).catch( err => console.log(err) );
  
          if (yt_resoult && yt_resoult.length > 0) {
            const song: Song = {
              name: yt_resoult[0].title as string,
              url: yt_resoult[0].url,
              length: yt_resoult[0].durationRaw,
              user_name: user_name
            }
          
            this.songQueue.push(song);
  
            message.channel.send(`${song.name} has been added to the queue.`);
          }
          else {
            message.reply("Requested song could not be found.");
          }
        }
        else if (raw_resoults.type === "playlist" || raw_resoults.type === "album") {
          let missed_songs = 0;
          let raw_resoults2;

          if ( raw_resoults.type === "playlist" ) {
            raw_resoults2 = raw_resoults as SpotifyPlaylist
          }
          else {
            raw_resoults2 = raw_resoults as SpotifyAlbum
          }

          (await raw_resoults2.all_tracks()).map(async (raw_song) => {
            const yt_resoult = await playdl.search(raw_song.name + " lyrics", { limit: 1 }).catch( err => console.log(err) );
  
            if (yt_resoult && yt_resoult.length > 0) {
              const song: Song = {
                name: yt_resoult[0].title as string,
                url: yt_resoult[0].url,
                length: yt_resoult[0].durationRaw,
                user_name: user_name
              }
    
              this.songQueue.push(song);
            }
            else {
              message.reply(`\`${raw_resoults.type === "playlist" ? raw_resoults.name: ""}\` could not be found`);
              missed_songs++;
            }
          });
  
          message.channel.send(`\`${raw_resoults2.tracksCount - missed_songs}\` songs added to the queue`);
        }
      }
      else {
        message.reply("We cannot found anything with this link. Thw link may be broken.");
      }
    }
    else if (argument.search("list=") === -1) {
      console.log("URL");
      const raw_resoults = await playdl.video_info(argument).catch( err => console.log(err) );

      if (raw_resoults) {
        const song: Song = {
          name: raw_resoults.video_details.title as string,
          url: argument,
          length: raw_resoults.video_details.durationRaw,
          user_name: user_name
        }
        
        this.songQueue.push(song);
        
        message.channel.send(`${song.name} has been added to the queue.`);
      }
      else {
        message.reply("Requested song could not be found. Link may be broken, from hidden video or from unsported source.");
      }
    }
    else {
      console.log("PLAYLIST");
      const raw_resoults = await playdl.playlist_info(argument, { incomplete: true }).catch( err => console.log(err) );

      if (raw_resoults) {
        const raw_resoults2 = raw_resoults.toJSON();
        
        if (raw_resoults2.videos) {
          raw_resoults2.videos.map((raw_song) =>{
            const song: Song = {
              name: raw_song.title as string,
              url: raw_song.url,
              length: raw_song.durationRaw,
              user_name: user_name
            }
            this.songQueue.push(song);
          });
  
          message.channel.send(`**${raw_resoults2.videos.length}** songs added to queue.`);
        }
        else {
          message.reply("Error happened while looking to playlist.");
        }
      }
      else {
        message.reply("Requested playlist could not be found. It may be hidden or from unsported source.");
      }
    }

    if (!this.now_playing) {
      this.start();
    }
  }

  public async stop(message?: Message) {
    // this.connection.disconnect();

    this.now_playing = undefined as unknown as Song;
    this.songQueue = [];

    this.player.stop();

    if (message) {
      message.reply("Goodbye. :sob: ");
    }
  }

  public async skip(message: Message) {
    if (this.now_playing) {
      message.channel.send(`\`${this.now_playing.name}\` is skipped`);
    }
    else {
      message.reply("We cannot skip. Nothings playing.");
    }

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

    message.channel.send("Queue is shuffled. (You cannot undo shuffleing.)");
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

    message.channel.send(reply_message);
  }

  private async changeStream(url: string) {
    console.log("Now playing: ", url);
    this.stream = await playdl.stream(url, this.stream_options);
    this.resource = Voice.createAudioResource(this.stream.stream, { inputType: this.stream.type });
    this.player.play(this.resource);
    this.connection.subscribe(this.player);
  }

  private start() {
    if (this.songQueue.length > 0) {
      this.now_playing = this.songQueue.shift() as Song;

      this.changeStream(this.now_playing.url);
    }
    else {
      this.stop();
    }
  }
}

export default Player;
