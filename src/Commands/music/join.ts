import { ApplicationCommandOptionTypes } from "discord.js/typings/enums";
import { Command } from "../../Interfaces";

export const command: Command = {
  name: "join",
  description: "Joins to the voice channel",
  category: "Music",
  options: [{
    name: "channel",
    description: "the channel that will be connected",
    type: ApplicationCommandOptionTypes.CHANNEL,
    required: false
  }],
  aliases: ["j"],
  run: (variables) => {
    variables.client.player.joinVC(variables);
  }
}
