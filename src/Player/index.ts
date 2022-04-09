// Discord API
import { GuildMember } from "discord.js";
import { AudioPlayer, AudioPlayerStatus, AudioResource,
         createAudioPlayer, createAudioResource, DiscordGatewayAdapterCreator,
         joinVoiceChannel, NoSubscriberBehavior, VoiceConnection } from "@discordjs/voice";

// YouTube API
import playdl, { SpotifyAlbum, SpotifyPlaylist, SpotifyTrack,
                 YouTubeStream, YouTubeVideo } from "play-dl";

// Interfaces
import { RepeatOptions, Song, SpotifyConfig, StreamOptions, Variables } from "../Interfaces";

class Player {
  private song_queue:    Array<Song>   = [];
  private repeat_queue:  Array<Song>   = [];
  private now_playing:   Song | null   = null;
  private is_paused:     Boolean       = false;
  private can_use_sp:    Boolean       = false;
  private repeat_option: RepeatOptions = "None";

  private player:        AudioPlayer = createAudioPlayer({ behaviors: { noSubscriber: NoSubscriberBehavior.Play }});
  private resource:      AudioResource;
  private connection:    VoiceConnection;
  private stream:        YouTubeStream;

  private stream_options: StreamOptions = {
    language: "en-US",
    quality: 2,
    discordPlayerCompatibility: true
  };

  private user_agent_list = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.84 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.74 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.74 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.74 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.82 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.82 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.82 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.82 Safari/537.36"
  ];

  constructor() {
    this.player.on("error", (err) => {
      console.error(err);
      this.start();
    });

    this.player.on(AudioPlayerStatus.Idle, () => {
        this.start();
    });

    this.set_user_agents(this.user_agent_list);
  }

  private async set_user_agents(agents: string[]) {
    await playdl.setToken({
      useragent: agents
    });
  }

  public async set_yt_cookie(cookie: string) {
    await playdl.setToken({
      youtube: {
        cookie: cookie
      }
    });
  }

  public async set_sp_tokens(cfg: SpotifyConfig) {
    await playdl.setToken({
      spotify: {
        ...cfg,
        market: 'US'
      }
    });

    this.can_use_sp = true;
  }

  public async joinVC(variables: Variables): Promise<boolean> {
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

        if (this.connection) { return true; }
        else { return false; }
      } else {
        await variables.client.messager.send_err(variables,
               "Failed to join to voice channel. (Posibly you are not in a voice channel.)",
               "Failed to join to voice channel");

        return false;
      }
    } else {
      console.log("RECONNECTING");
      const sonnection = this.connection.rejoin();
      return sonnection;
    }
  }

  public async play(variables: Variables, url?: string) {
    if (!await this.joinVC(variables)) { return; }

    const main = variables.type === "Old" ? variables.message : variables.interaction;
    const argument  = url ? url 
                    : variables.type === "Old" ? variables.args.join(" ")
                    : variables.interaction.options.getString("song")!;
    const user = (main.member as GuildMember).displayName;

    if (argument.search("http://")  === -1 &&
        argument.search("https://") === -1 &&
        argument.search("www.")     === -1)  {
      // Search by word
      await this.handle_search(variables, argument);
      return;
    } else if(argument.search("open.spotify.com") !== -1) {
      // Spotify link
      await this.handle_spotify(variables, argument, user);
    } else if (argument.search("youtube.com") !== -1) {
      // Youtube link
      await this.handle_youtube(variables, argument, user);
    } else {
      await variables.client.messager.send_err(variables,
             "Invalid URL.", "Took invalid URL: " + url);
      return;
    }

    if (!this.now_playing) {
      this.start();
    }
  }

  public async pause(variables: Variables) {
    if (!this.now_playing) {
      await variables.client.messager.send_err(variables, "Nothings playing.");
      return;
    }

    if (!this.is_paused) {
      this.stream.pause();
      this.is_paused = true;
      await variables.client.messager.send_sucsess(variables, "Player is paused.");
    } else {
      await variables.client.messager.send_err(variables, "Player is already paused! :angry:");
    }
  }

  public async resume(variables: Variables) {
    if (!this.now_playing) {
      await variables.client.messager.send_err(variables, "Nothings playing.");
      return;
    }

    if (this.is_paused) {
      this.stream.pause();
      this.is_paused = true;
      await variables.client.messager.send_sucsess(variables, "Player is resumed.");
    } else {
      await variables.client.messager.send_err(variables, "Player is already playing! :angry:");
    }
  }

  public async stop(variables?: Variables) {
    this.now_playing  = null;
    this.song_queue   = [];
    this.repeat_queue = [];

    this.player.stop();

    if (variables) {
      await variables.client.messager.send_normal(variables, "Goodbye", ":sob:");
    }
  }

  public async skip(variables: Variables) {
    if (this.now_playing) {
      await variables.client.messager.send_sucsess(variables,
             `\`${this.now_playing.name}\` is skipped`);
      this.start();
    } else {
      await variables.client.messager.send_err(variables, "We cannot skip. Nothings playing.");
    }
  }

  public async repeat(variables: Variables, option?: RepeatOptions) {
    const argument  = (option ? option 
                    : variables.type === "Old" ? variables.args.join(" ")
                    : variables.interaction.options.getString("option")!)
                    .toLowerCase();
    if (argument) {
      switch (argument) {
        case "none":
          this.repeat_option = "None";
          break;
        case "one":
          this.repeat_option = "One";
          break;
        case "all":
          this.repeat_option = "All";
          break;
        default:
          await variables.client.messager.send_err(variables, "Invalid option.");
          return;
      }
      await variables.client.messager.send_sucsess(variables, `Repeat is changed to ${argument}.`);
    } else {
      const list = [
        {name: "None", id: "None", disabled: this.repeat_option === "None"},
        {name: "One",  id: "One",  disabled: this.repeat_option === "One"},
        {name: "All",  id: "All",  disabled: this.repeat_option === "All"},
      ];
      const content = `Current repeat ooption is \`${this.repeat_option}\`. Select one to change:`;
      
      await variables.client.messager.send_selection(variables, list, this.repeat, variables.client.player, "Repeat", content);
    }
  }

  public async shuffle(variables: Variables) {
    if (this.song_queue.length === 0 &&
        this.repeat_queue.length === 0) {
      await variables.client.messager.send_err(variables, "Queue is empty");
      return;
    }

    // Add everything to main queue if it is repeating
    if (this.repeat_option === "All") {
      this.song_queue.push(...this.repeat_queue);
      this.repeat_queue = [];
    }

    // The modern version of the Fisher–Yates shuffle algorithm
    for(let i = this.song_queue.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      const tmp = this.song_queue[i];
      this.song_queue[i] = this.song_queue[j];
      this.song_queue[j] = tmp;
    }

    await variables.client.messager.send_sucsess(variables,
           "Queue is shuffled.", "Queue shuffled");
  }

  public async queue(variables: Variables) {
    if (!this.now_playing) {
      await variables.client.messager.send_err(variables, "Nothings playing. :unamused: ");
      return;
    }

    let reply_message = `Currently playing \`${this.now_playing.name}\` [${this.now_playing.length}], requested by **${this.now_playing.user_name}**\n`;
    const queue_length = this.song_queue.length;

    if (queue_length <= 10) {
      for (let i = 0; i < queue_length; i++) {
        reply_message += `**${i + 1}** \`${this.song_queue[i].name}\` [${this.song_queue[i].length}]\n`;
      }
    } else {
      for (let i = 0; i <= 10; i++) {
        reply_message += `**${i + 1}** \`${this.song_queue[i].name}\` [${this.song_queue[i].length}]\n`;
      }
      reply_message += `And ${queue_length - 10} more...`;
    }

    await variables.client.messager.send_normal(variables, "Queue", reply_message);
  }

  private async change_stream() {
    if (!this.now_playing) { return; }
    console.log("Now playing: ", this.now_playing.url);
    this.stream = await playdl.stream(this.now_playing.url, this.stream_options);
    this.resource = createAudioResource(this.stream.stream, { inputType: this.stream.type });
    this.player.play(this.resource);
    this.connection.subscribe(this.player);
  }

  private start() {
    switch (this.repeat_option) {
      case "One":
        if (!this.now_playing &&
            this.song_queue.length > 0) {
          this.now_playing = this.song_queue.shift()!;
        } else if (!this.now_playing &&
                   this.song_queue.length === 0) {
          this.stop();
          break;
        }

        this.change_stream();
        break;
      case "All":
        if (this.song_queue.length === 0 &&
            this.repeat_queue.length === 0 &&
            !this.now_playing) {
          this.stop();
          break;
        }

        if (this.now_playing) {
          this.repeat_queue.push(this.now_playing);
        }

        if (this.song_queue.length === 0) {
          this.song_queue = this.repeat_queue;
          this.repeat_queue = [];
        }

        this.now_playing = this.song_queue.shift()!;
        this.change_stream();
        break;
      case "None":
        if (this.song_queue.length > 0) {
          this.now_playing = this.song_queue.shift()!;
          this.change_stream();
        } else {
          this.stop();
        }
        break;
      default:
        console.log("Impossible repeat block.");
    }
  }

  private async handle_search(variables: Variables, argument: string) {
    const raw_resoults = await playdl.search(argument, { source: { youtube: "video" }, limit: 5 })
        .catch( err => console.error(err) );

    if (raw_resoults && raw_resoults.length > 0) {
      const list = raw_resoults.map((element) => {
        return {
          name: element.title as string,
          id: element.url,
          disabled: false
        };
      });

      variables.client.messager.send_selection_from_list(variables, list, true, this.play, variables.client.player, "Search");
    } else {
      await variables.client.messager.send_err(variables,
                "Requested song could not be found. Try to search with different key words.");
    }
  }

  private async handle_youtube(variables: Variables, argument: string, user: string) {
    if (argument.search("list=") === -1) {
      const raw_resoults = await playdl.video_info(argument)
          .catch( err => console.error(err) );

      if (raw_resoults) {
        this.push_to_queue(raw_resoults.video_details, user);
        await variables.client.messager.send_sucsess(variables,
                  `${raw_resoults.video_details.title} has been added to the queue.`);
      } else {
        await variables.client.messager.send_err(variables,
                  "Requested song could not be found. Link may be broken, from hidden video or from unsported source.");
      }
    } else {
      const raw_resoults = await playdl.playlist_info(argument, { incomplete: true })
          .catch( err => console.error(err) );

      if (raw_resoults) {
        const raw_resoults2 = raw_resoults.toJSON();
        
        if (raw_resoults2.videos) {
           await variables.client.messager.send_normal(variables,
                           "Started", "Started to add songs to queue");

          raw_resoults2.videos.forEach((raw_song) => {
            this.push_to_queue(raw_song, user);
          });

          await variables.client.messager.send_sucsess(variables,
                    `**${raw_resoults2.videos.length}** songs added to queue.`);
        } else {
          await variables.client.messager.send_err(variables,
                    "Error happened while looking to playlist.");
        }
      } else {
        await variables.client.messager.send_err(variables,
                  "Requested playlist could not be found. It may be hidden or from unsported source.");
      }
    }
  }

  private async handle_spotify(variables: Variables, argument: string, user: string) {
    if (!this.can_use_sp) {
      await variables.client.messager.send_err(variables,
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
        const raw_resoults2 = raw_resoults as SpotifyTrack;

        const search_string = raw_resoults2.artists[0].name + " - " + raw_resoults2.name + " lyrics";
        const yt_resoult = await playdl.search(search_string, { source: { youtube: "video" }, limit: 1 })
            .catch( err => console.error(err) );

        if (yt_resoult && yt_resoult.length > 0) {
          this.push_to_queue(yt_resoult[0], user);
          await variables.client.messager.send_sucsess(variables,
                    `${yt_resoult[0].title} has been added to the queue.`);
        } else {
          await variables.client.messager.send_err(variables,
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

        await variables.client.messager.send_normal(variables,
                  "Started", "Started to add songs to queue");

        const wait = track_list.map((raw_song) => {
          const search_string = raw_song.artists[0].name + " - " + raw_song.name + " lyrics";
          return playdl.search(search_string, { source: { youtube: "video" }, limit: 1 });
        });

        await Promise.all(wait).then((awaited_resoults) => awaited_resoults.forEach((yt_resoult) => {
          if (yt_resoult.length > 0) {
            this.push_to_queue(yt_resoult[0], user);
          } else {
            variables.client.messager.send_err(variables,
                `\`${raw_resoults.name}\` could not be found`);
            missed_songs++;
          }
        })).catch((err) => {
          variables.client.messager.send_err(variables,
              "An error accured while opening the "
              + raw_resoults.type === "playlist" ? "playlist"
                                                 : "album");
          console.log(err);
        });

        await variables.client.messager.send_sucsess(variables,
                  `\`${raw_resoults2.tracksCount - missed_songs}\` songs added to the queue`);
      }
    } else {
      await variables.client.messager.send_err(variables,
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

    this.song_queue.push(song);
  }
}

export default Player;
