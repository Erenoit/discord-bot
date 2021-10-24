import { Command } from "../../Interfaces";

export const command: Command = {
  name: "queue",
  description: "Shows the queue",
  aliases: ["q"],
  run: (client, message, args) => {
    client.player.queue(message);
  }
}
