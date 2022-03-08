import { Command } from "../../Interfaces";

export const command: Command = {
  name: "shuffle",
  description: "Adds song to queue",
  aliases: [],
  run: (client, message, args) => {
    client.player.shuffle(message);
  }
}
