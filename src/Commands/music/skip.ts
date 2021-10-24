import { Command } from "../../Interfaces";

export const command: Command = {
  name: "skip",
  description: "Skips the current playing music",
  aliases: ["s"],
  run: (client, message, args) => {
    client.player.skip(message);
  }
}

