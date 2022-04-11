import { ApplicationCommandOptionTypes } from "discord.js/typings/enums";
import { Command } from "../../Interfaces";

export const command: Command = {
  name: "play",
  description: "Adds song to queue",
  category: "Music",
  options: [{
    name: "song",
    description: "Song name or Song URL",
    type: ApplicationCommandOptionTypes.STRING,
    required: true
  }],
  aliases: ["p"],
  run: (variables) => {
    variables.client.player.play(variables);
  }
}
