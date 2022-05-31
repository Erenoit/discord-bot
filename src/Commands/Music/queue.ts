import { Command } from "../../Interfaces";

export const command: Command = {
  name: "queue",
  description: "Shows the queue",
  category: "Music",
  aliases: ["q"],
  run: (variables) => {
    variables.client.player.queue(variables);
  }
}