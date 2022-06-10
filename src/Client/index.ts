import { ApplicationCommandManager, Client,
         Collection, GuildApplicationCommandManager,
         Snowflake } from "discord.js";
import { readdirSync } from "fs";
import path from "path";
import dotenv from "dotenv";

import { Command, Config, Event, Server } from "../Interfaces";
import Player from "../Player";
import Messager from "../Messager";



class MyClient extends Client {
  public commands: Collection<string, Command>   = new Collection();
  public events:   Collection<string, Event>     = new Collection();
  public aliases:  Collection<string, Command>   = new Collection();
  public servers:  Collection<Snowflake, Server> = new Collection();
  public messager: Messager                      = new Messager();
  public config:   Config;

  public async init() {

    // Generate environmental variables
    this.generate_config();

    // Commands
    this.init_commands();

    // Events
    this.init_events();

    // Login
    this.login(this.config.token);
  }

  private generate_config() {
    dotenv.config();

    // Check for required variables
    const token  = process.env.TOKEN;
    const prefix = process.env.PREFIX;
    if (!token) {
      console.log("You must have TOKEN environment variable as your bots token.");
      return;
    }
    if (!prefix) {
      console.log("You must have PREFIX environment variable as your bots prefix.");
      return;
    }

    this.config = {
      token,
      prefix
    }

    const yt_cookie = process.env.YT_COOKIE;

    if (yt_cookie) {
      this.config = {
        ...this.config,
        yt_cookie
      }
    }

    const client_id     = process.env.SP_CLIENT_ID;
    const client_secret = process.env.SP_CLIENT_SECRET;
    const refresh_token = process.env.SP_REFRESH_TOKEN;

    if (client_id && client_secret && refresh_token) {
      this.config = {
        ...this.config,
        spotify: {
          client_id,
          client_secret,
          refresh_token
        }
      }
    }
  }

  private init_commands() {
    console.log("----- Generating Commands -----");
    const command_path = path.join(__dirname, "..", "Commands");
    readdirSync(command_path).forEach((dir) => {
      const commands = readdirSync(`${command_path}/${dir}`);

      commands.forEach((file) => {
        const { command } = require(`${command_path}/${dir}/${file}`);
        this.commands.set(command.name, command);

        console.log(command);

        if (command?.aliases.length > 0) {
          command.aliases.forEach((alias: string) => {
            this.aliases.set(alias, command);
          });
        }
      });
    });
  }

  private init_events() {
    console.log("----- Generating Events -----");
    const event_path = path.join(__dirname, "..", "Events");
    readdirSync(event_path).forEach(async (file) => {
      const { event } = await import(`${event_path}/${file}`);
      this.events.set(event.name, event);

      console.log(event);

      this.on(event.name, event.run.bind(null, this));
    });
  }

  public register_commands(isTesting: Boolean = false) {
    let command_manager: GuildApplicationCommandManager | ApplicationCommandManager;

    if (isTesting) {
      command_manager = this.guilds.cache.get("697571152280682615")!.commands;
    } else {
      command_manager = this.application!.commands;
    }

    this.commands.forEach(async (command) => {
      await command_manager.create({
        name: command.name,
        description: command.description,
        options: command.options
      });
    });
  }

  public register_servers() {
    this.guilds.cache.forEach((server) => {
      const s: Server = {
        guild_id: server.id,
        prefix: this.config.prefix,
        player: new Player()
      };

      this.servers.set(server.id, s);
    });
  }

  public register_yt_cookie() {
    if (this.config.yt_cookie) {
      console.log("---------------- Youtube Cookies Found ----------------");
      this.servers.forEach((server) => {
        server.player.set_yt_cookie(this.config.yt_cookie!);
      });
    }
  }

  public register_sp_tokens() {
    if (this.config.spotify) {
      console.log("---------------- Spotify Configs Found ----------------");
      this.servers.forEach((server) => {
        server.player.set_sp_tokens(this.config.spotify!);
      });
    }
  }
}

export default MyClient;
