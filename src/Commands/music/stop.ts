import { Command } from "../../Interfaces";

export const command: Command = {
  name: "stop",
  description: "Stops the music",
  aliases: ["st"],
  run: (client, message, args) => {
    client.player.stop(message);
  }
}
