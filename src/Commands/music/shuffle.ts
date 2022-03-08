import { Command } from "../../Interfaces";

export const command: Command = {
  name: "shuffle",
  description: "Adds song to queue",
  aliases: [],
  run: (variables) => {
    variables.client.player.shuffle(variables);
  }
}
