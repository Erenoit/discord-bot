import { Command } from "../../Interfaces";

export const command: Command = {
  name: "join",
  description: "Joins to the voice channel",
  aliases: ["j"],
  run: (client, message, args) => {
    client.player.joinVC(message, args);
  }
}