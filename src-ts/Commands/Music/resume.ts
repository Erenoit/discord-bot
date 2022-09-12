import { Command } from "../../Interfaces";

export const command: Command = {
  name: "resume",
  description: "Resumes the player",
  category: "Music",
  aliases: [],
  run: (variables) => {
    variables.client.servers.get(variables.guild_id)?.player.resume(variables);
  }
}
