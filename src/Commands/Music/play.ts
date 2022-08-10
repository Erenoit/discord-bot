import { ApplicationCommandOptionType } from "discord.js";
import { Command } from "../../Interfaces";

export const command: Command = {
  name: "play",
  description: "Adds song to queue",
  category: "Music",
  options: [{
    name: "song",
    description: "Song name or Song URL",
    type: ApplicationCommandOptionType.String,
    required: true
  }],
  aliases: ["p"],
  run: (variables) => {
    variables.client.servers.get(variables.guild_id)?.player.play(variables);
  }
}
