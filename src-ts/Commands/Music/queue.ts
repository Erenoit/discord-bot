import { Command } from "../../Interfaces";

export const command: Command = {
  name: "queue",
  description: "Shows the queue",
  category: "Music",
  aliases: ["q"],
  run: (variables) => {
    variables.client.servers.get(variables.guild_id)?.player.queue(variables);
  }
}
