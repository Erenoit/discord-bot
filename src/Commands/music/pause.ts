import { Command } from "../../Interfaces";

export const command: Command = {
  name: "pause",
  description: "Pauses the player",
  category: "Music",
  aliases: [],
  run: (variables) => {
    variables.client.player.pause(variables);
  }
}
