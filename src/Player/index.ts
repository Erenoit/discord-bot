// Discord API
import { Collection, GuildMember } from "discord.js";
import { AudioPlayer, AudioPlayerStatus, AudioResource,
         createAudioPlayer, createAudioResource, DiscordGatewayAdapterCreator,
         joinVoiceChannel, NoSubscriberBehavior, VoiceConnection } from "@discordjs/voice";

// YouTube API
import playdl, { SpotifyAlbum, SpotifyPlaylist, YouTubeStream, YouTubeVideo } from "play-dl";

// Interaces
import { PlayerEvent, Song, StreamOptions, Variables } from "../Interfaces";

class Player {
  public  events:      Collection<string, PlayerEvent> = new Collection()
  private songQueue:   Array<Song> = [];
  private now_playing: Song;
  private can_use_sp:  Boolean = false;

  private player:      AudioPlayer = createAudioPlayer({ behaviors: { noSubscriber: NoSubscriberBehavior.Play }});
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
      console.error(err);
      this.start();
    });

    this.player.on(AudioPlayerStatus.Idle, () => {
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

    this.can_use_sp = true;
  }

  public joinVC(variables: Variables) {
    if (!this.connection) {
      console.log("FIRST TIME JOIN");

      const main = variables.type === "Old" ? variables.message : variables.interaction;;
      const channelID = (main.member as GuildMember).voice?.channel?.id
      const guildID   = main.guild?.id;
      const adapter   = main.guild?.voiceAdapterCreator as unknown as DiscordGatewayAdapterCreator;

      if(channelID && guildID && adapter) {
        this.connection = joinVoiceChannel({
          channelId: channelID,
          guildId: guildID,
          adapterCreator: adapter,
          selfDeaf: true
        });
      } else {
        variables.client.messager.send_err(variables,
          "Failed to join to voice channel. (Posibly you are not in a voice channel.)",
          "Failed to join to voice channel");
      }
    } else {
      console.log("RECONNECTING");
      const sonnection = this.connection.rejoin();
      console.log(sonnection);
    }
  }

  public async play(variables: Variables, url?: string) {
    this.joinVC(variables);

    const main = variables.type === "Old" ? variables.message : variables.interaction;
    const argument  = url ? url 
                    : variables.type === "Old" ? variables.args.join(" ")
                    : variables.interaction.options.getString("song")!;
    const user = (main.member as GuildMember).displayName;

    if (argument.search("http://")  === -1 &&
        argument.search("https://") === -1 &&
        argument.search("www.")      === -1)  {
      // Search by word
      await this.handle_search(variables, argument, user);
    } else if(argument.search("open.spotify.com") !== -1) {
      // Spotify link
      await this.handle_spotify(variables, argument, user);
    } else if (argument.search("youtube.com") !== -1) {
      // Youtube link
      await this.handle_youtube(variables, argument, user);
    } else {
      variables.client.messager.send_err(variables,
                                         "Invalid URL.",
                                         "Took invalid URL: "+url);
      return;
    }

    if (!this.now_playing) {
      this.start();
    }
  }

  public async stop(variables?: Variables) {
    // this.connection.disconnect();

    this.now_playing = undefined as unknown as Song;
    this.songQueue = [];

    this.player.stop();

    if (variables) {
      variables.client.messager.send_normal(variables, "Goodbye", ":sob:");
    }
  }

  public async skip(variables: Variables) {
    if (this.now_playing) {
      variables.client.messager.send_sucsess(variables, `\`${this.now_playing.name}\` is skipped`);
      this.start();
    } else {
      variables.client.messager.send_err(variables, "We cannot skip. Nothings playing.");
    }
  }

  public async shuffle(variables: Variables) {
    // The modern version of the Fisher–Yates shuffle algorithm
    for(let i = this.songQueue.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      const tmp = this.songQueue[i];
      this.songQueue[i] = this.songQueue[j];
      this.songQueue[j] = tmp;
    }

    // TODO: add 'Are you sure?' with buttons
    variables.client.messager.send_sucsess(variables,
                                           "Queue is shuffled. (You cannot undo shuffleing.)",
                                           "Queue shuffled");
  }

  public async queue(variables: Variables) {
    if (!this.now_playing) {
      variables.client.messager.send_err(variables, "Nothings playing. :unamused: ");
      return;
    }

    let reply_message = `Currently playing \`${this.now_playing.name}\` [${this.now_playing.length}], requested by **${this.now_playing.user_name}**\n`;
    const queue_length = this.songQueue.length;

    if (queue_length <= 10) {
      for (let i = 0; i < queue_length; i++) {
        reply_message += `**${i}** \`${this.songQueue[i].name}\` [${this.songQueue[i].length}]\n`;
      }
    } else {
      for (let i = 0; i <= 10; i++) {
        reply_message += `**${i}** \`${this.songQueue[i].name}\` [${this.songQueue[i].length}]\n`;
      }
      reply_message += `And ${queue_length - 10} more...`;
    }

    variables.client.messager.send_normal(variables, "Queue", reply_message);
  }

  private async changeStream(url: string) {
    console.log("Now playing: ", url);
    this.stream = await playdl.stream(url, this.stream_options);
    this.resource = createAudioResource(this.stream.stream, { inputType: this.stream.type });
    this.player.play(this.resource);
    this.connection.subscribe(this.player);
  }

  private start() {
    if (this.songQueue.length > 0) {
      this.now_playing = this.songQueue.shift() as Song;

      this.changeStream(this.now_playing.url);
    } else {
      this.stop();
    }
  }

  private async handle_search(variables: Variables, argument: string, user: string) {
    const raw_resoults = await playdl.search(argument, { limit: 1 })
        .catch( err => console.error(err) );

    if (raw_resoults && raw_resoults.length > 0) {
      this.push_to_queue(raw_resoults[0], user);
      variables.client.messager.send_sucsess(variables,
                `${raw_resoults[0].title} has been added to the queue.`);
    } else {
      variables.client.messager.send_err(variables,
                "Requested song could not be found. Try to search with different key words.");
    }
  }

  private async handle_youtube(variables: Variables, argument: string, user: string) {
    if (argument.search("list=") === -1) {
      const raw_resoults = await playdl.video_info(argument)
          .catch( err => console.error(err) );

      if (raw_resoults) {
        this.push_to_queue(raw_resoults.video_details, user);
        variables.client.messager.send_sucsess(variables,
                  `${raw_resoults.video_details.title} has been added to the queue.`);
      } else {
        variables.client.messager.send_err(variables,
                  "Requested song could not be found. Link may be broken, from hidden video or from unsported source.");
      }
    } else {
      const raw_resoults = await playdl.playlist_info(argument, { incomplete: true })
          .catch( err => console.error(err) );

      if (raw_resoults) {
        const raw_resoults2 = raw_resoults.toJSON();
        
        if (raw_resoults2.videos) {
          variables.client.messager.send_normal(variables,
                    "Started", "Started to add songs to queue");

          raw_resoults2.videos.map((raw_song) =>{
            this.push_to_queue(raw_song, user);
          });

          variables.client.messager.send_sucsess(variables,
                    `**${raw_resoults2.videos.length}** songs added to queue.`);
        } else {
          variables.client.messager.send_err(variables, "Error happened while looking to playlist.");
        }
      } else {
        variables.client.messager.send_err(variables,
                  "Requested playlist could not be found. It may be hidden or from unsported source.");
      }
    }
  }

  private async handle_spotify(variables: Variables, argument: string, user: string) {
    if (!this.can_use_sp) {
      variables.client.messager.send_err(variables,
                "Bot is not logined to spotify. Please request from bot's administrator.",
                "Spotify support wanted");
      return;
    }
    if (playdl.is_expired()) {
      await playdl.refreshToken();
    }

    const raw_resoults = await playdl.spotify(argument)
        .catch( err => console.error(err) );

    if (raw_resoults) {
      if (raw_resoults.type === "track") {
        const yt_resoult = await playdl.search(raw_resoults.name + " lyrics", { limit: 1 })
            .catch( err => console.error(err) );

        if (yt_resoult && yt_resoult.length > 0) {
          this.push_to_queue(yt_resoult[0], user);
          variables.client.messager.send_sucsess(variables,
                    `${yt_resoult[0].title} has been added to the queue.`);
        } else {
          variables.client.messager.send_err(variables,
                    "Requested song could not be found.");
        }
      } else if (raw_resoults.type === "playlist" || raw_resoults.type === "album") {
        let missed_songs = 0;
        let raw_resoults2;

        if (raw_resoults.type === "playlist" ) {
          raw_resoults2 = raw_resoults as SpotifyPlaylist
        } else {
          raw_resoults2 = raw_resoults as SpotifyAlbum
        }

        const track_list = await raw_resoults2.all_tracks();

        variables.client.messager.send_normal(variables,
                  "Started", "Started to add songs to queue");

        // Couldn't use arr.map() because I'm using await in iteration
        for (let i = 0; i < track_list.length; i++) {
          const raw_song = track_list[i];
          const yt_resoult = await playdl.search(raw_song.name + " lyrics", { limit: 1 })
              .catch( err => console.error(err) );

          if (yt_resoult && yt_resoult.length > 0) {
            this.push_to_queue(yt_resoult[0], user);
          } else {
            variables.client.messager.send_err(variables,
                      `\`${raw_resoults.name}\` could not be found`);
            missed_songs++;
          }

          // If the bot is not playing start playing 
          // without waiting to finish adding all the elements
          if (i === 0 && !this.now_playing) {
            this.start();
          }
        };

        variables.client.messager.send_sucsess(variables,
                  `\`${raw_resoults2.tracksCount - missed_songs}\` songs added to the queue`);
      }
    } else {
      variables.client.messager.send_err(variables,
                "We cannot found anything with this link. Thw link may be broken.");
    }
  }

  private push_to_queue(s: YouTubeVideo, user_name: string) {
    const song: Song = {
      name: s.title as string,
      url: s.url,
      length: s.durationRaw,
      user_name: user_name
    }

    this.songQueue.push(song);
  }
}

export default Player;
