import { Command } from "../../Interfaces";

export const command: Command = {
  name: "stop",
  description: "Stops the music",
  category: "Music",
  aliases: ["st"],
  run: (variables) => {
    variables.client.player.stop(variables);
  }
}
