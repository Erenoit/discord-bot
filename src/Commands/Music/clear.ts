import { Command } from "../../Interfaces";

export const command: Command = {
  name: "clear",
  description: "Clears the queue",
  category: "Music",
  aliases: [],
  run: (variables) => {
    variables.client.player.clear(variables);
  }
}

