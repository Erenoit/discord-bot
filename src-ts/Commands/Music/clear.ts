import { Command } from "../../Interfaces";

export const command: Command = {
  name: "clear",
  description: "Clears the queue",
  category: "Music",
  aliases: [],
  run: (variables) => {
    variables.client.servers.get(variables.guild_id)?.player.clear(variables);
  }
}

