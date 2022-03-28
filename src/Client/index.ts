import { ApplicationCommandManager, Client,
         Collection, GuildApplicationCommandManager } from "discord.js";
import { readdirSync } from "fs";
import path from "path";
import dotenv from "dotenv";

import { Command, Event } from "../Interfaces";
import Player from "../Player";
import Messager from "../Messager";



class MyClient extends Client {
  public commands: Collection<string, Command> = new Collection();
  public events:   Collection<string, Event>   = new Collection();
  public aliases:  Collection<string, Command> = new Collection();
  public player:   Player                      = new Player();
  public messager: Messager                    = new Messager();  
  public prefix:   string;

  public async init() {

    // Generate environmental variables
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
    
    // Set prefix
    this.prefix = prefix;

    // Commands
    this.init_commands();

    // Events
    this.init_events();

    // YouTube
    const yt_cookie = process.env.YT_COOKIE;
    if (yt_cookie) {
      console.log("---------------- Youtube Cookies Found ----------------");
      this.player.setYTCookie(yt_cookie);
    }

    // Spotify
    const sp_client_id     = process.env.SP_CLIENT_ID;
    const sp_client_secret = process.env.SP_CLIENT_SECRET;
    const sp_refresh_token = process.env.SP_REFRESH_TOKEN;
    if (sp_client_id && sp_client_secret && sp_refresh_token) {
      console.log("---------------- Spotify Configs Found ----------------");
      this.player.setSPToken(sp_client_id, sp_client_secret, sp_refresh_token);
    }

    // Login
    this.login(token);
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
}

export default MyClient;
