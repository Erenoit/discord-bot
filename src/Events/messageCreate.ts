import { Event, Command } from "../Interfaces";
import { Message } from "discord.js";

export const event: Event = {
  name: "messageCreate",
  run: async (client, message: Message) => {
    if (
      message.author.bot ||
      !message.guild     ||
      !message.content.startsWith(client.config.prefix)
    ) { return; }

    const [cmd, ...args] = message.content
      .slice(client.config.prefix.length)
      .trim()
      .split(/ +/g);

    if (!cmd) { return; }
    console.log(`Command: ${cmd}`);

    const command = client.commands.get(cmd.toLowerCase()) || client.aliases.get(cmd.toLowerCase());
    if (command) { (command as Command).run(client, message, args); }
    else { message.reply("We do not have that command! :angry:") }
  }
};
