import { Command } from "../../Interfaces";

export const command: Command = {
  name: "skip",
  description: "Skips the current playing music",
  category: "Music",
  aliases: ["s"],
  run: (variables) => {
    variables.client.servers.get(variables.guild_id)?.player.skip(variables);
  }
}

