import { Client, Collection } from "discord.js";
import { readdirSync } from "fs";
import path from "path";

import { Command, Event, Config } from "../Interfaces";
import Player from "../Player";
import configjson from "../config.json";



class MyClient extends Client {
  public commands: Collection<string, Command> = new Collection();
  public events:   Collection<string, Event>   = new Collection();
  public aliases:  Collection<string, Command> = new Collection();
  public player:   Player                      = new Player();
  public config:   Config                      = configjson;

  public async init() {

    // Commands
    this.init_commands();

    // Events
    this.init_events();

    // Login
    this.login(this.config.token);
  }

  private init_commands() {
    console.log("----- Generating Commands -----");
    const command_path = path.join(__dirname, "..", "Commands");
    readdirSync(command_path).forEach((dir) => {
      const commands = readdirSync(`${command_path}/${dir}`);

      commands.map((file) => {
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
}

export default MyClient;
