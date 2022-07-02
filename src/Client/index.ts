import { ApplicationCommandManager, Client,
         Collection, GuildApplicationCommandManager,
         Snowflake } from "discord.js";
import { readdirSync } from "fs";
import path from "path";
import dotenv from "dotenv";

import { Command, Config, Event, Server } from "../Interfaces";
import Player from "../Player";
import Messager from "../Messager";
import Logger from "../Logger";



class MyClient extends Client {
  public commands: Collection<string, Command>   = new Collection();
  public events:   Collection<string, Event>     = new Collection();
  public aliases:  Collection<string, Command>   = new Collection();
  public servers:  Collection<Snowflake, Server> = new Collection();
  public messager: Messager                      = new Messager();
  public logger:   Logger                        = new Logger();
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
      this.logger.error("No Token", "You must have TOKEN environment variable as your bots token.");
      return;
    }
    if (!prefix) {
      this.logger.error("No Default Prefix", "You must have PREFIX environment variable as your bots prefix.");
      return;
    }

    this.logger.log("Registering Configs");

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
    this.logger.log("Generating Commands");
    const command_path = path.join(__dirname, "..", "Commands");
    readdirSync(command_path).forEach((dir) => {
      const commands = readdirSync(`${command_path}/${dir}`);

      commands.forEach((file) => {
        const { command } = require(`${command_path}/${dir}/${file}`);
        this.commands.set(command.name, command);

        this.logger.secondary_log(`${command.name}: ${command.description}`);

        if (command?.aliases.length > 0) {
          command.aliases.forEach((alias: string) => {
            this.aliases.set(alias, command);
          });
        }
      });
    });
  }

  private init_events() {
    this.logger.log("Generating Events");
    const event_path = path.join(__dirname, "..", "Events");
    readdirSync(event_path).forEach(async (file) => {
      const { event } = await import(`${event_path}/${file}`);
      this.events.set(event.name, event);

      this.logger.secondary_log(event.name);

      this.on(event.name, event.run.bind(null, this));
    });
  }

  public register_commands(isTesting: Boolean = false) {
    this.logger.log("Registering Commands");
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
    this.logger.log("Registering Servers");
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
      this.logger.log("Registering Youtube Cookies");
      this.servers.forEach((server) => {
        server.player.set_yt_cookie(this.config.yt_cookie!);
      });
    } else
      this.logger.warn("No Youtube Cookies Found");
  }

  public register_sp_tokens() {
    if (this.config.spotify) {
      this.logger.log("Registering Spotify Configs");
      this.servers.forEach((server) => {
        server.player.set_sp_tokens(this.config.spotify!);
      });
    } else
      this.logger.warn("No Spotify Configs Found");
  }
}

export default MyClient;
