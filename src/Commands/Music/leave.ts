import { Command } from "../../Interfaces";

export const command: Command = {
  name: "leave",
  description: "Leaves the voice channel",
  category: "Music",
  aliases: ["l"],
  run: (variables) => {
    variables.client.player.leaveVC(variables);
  }
}

