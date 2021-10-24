import { Command } from "../../Interfaces";

export const command: Command = {
  name: "pause",
  description: "Pauses the music",
  aliases: [],
  run: (client, message, args) => {
    client.player.pause(message);
  }
}
