import { Command } from "../../Interfaces";

export const command: Command = {
  name: "leave",
  description: "Leaves the voice channel",
  category: "Music",
  aliases: ["l"],
  run: (variables) => {
    variables.client.servers.get(variables.guild_id)?.player.leaveVC(variables);
  }
}

