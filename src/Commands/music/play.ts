import { Command } from "../../Interfaces";

export const command: Command = {
  name: "play",
  description: "Adds song to queue",
  aliases: ["p"],
  run: (client, message, args) => {
    client.player.play(message, args);
  }
}
