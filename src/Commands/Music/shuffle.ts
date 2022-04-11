import { Command } from "../../Interfaces";

export const command: Command = {
  name: "shuffle",
  description: "Adds song to queue",
  category: "Music",
  aliases: [],
  run: (variables) => {
    variables.client.messager.send_confirm(variables,
              variables.client.player.shuffle, variables.client.player, [variables],
              "You cannot undo shuffleing.");
    ;
  }
}
