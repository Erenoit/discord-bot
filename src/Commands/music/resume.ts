import { Command } from "../../Interfaces";

export const command: Command = {
  name: "resume",
  description: "Resumes music",
  aliases: ["r"],
  run: (client, message, args) => {
    client.player.resume(message);
  }
}

