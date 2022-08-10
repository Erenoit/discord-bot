import { ApplicationCommandOptionType } from "discord.js";
import { Command } from "../../Interfaces";

export const command: Command = {
  name: "join",
  description: "Joins the voice channel",
  category: "Music",
  options: [{
    name: "channel",
    description: "the channel that will be connected",
    type: ApplicationCommandOptionType.Channel,
    required: false
  }],
  aliases: ["j"],
  run: (variables) => {
    variables.client.servers.get(variables.guild_id)?.player.joinVC(variables);
  }
}
