import { Command } from "../../Interfaces";

export const command: Command = {
  name: "stop",
  description: "Stops the music",
  category: "Music",
  aliases: ["st"],
  run: (variables) => {
    variables.client.servers.get(variables.guild_id)?.player.stop(variables);
  }
}
