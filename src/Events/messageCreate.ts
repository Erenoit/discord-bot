import { Event, Variables } from "../Interfaces";
import { Message } from "discord.js";

export const event: Event = {
  name: "messageCreate",
  run: async (client, message: Message) => {
    const prefix = client.config.prefix;

    if (
      message.author.bot ||
      !message.guild     ||
      !message.content.startsWith(prefix)
    ) { return; }

    const [cmd, ...args] = message.content
      .slice(prefix.length)
      .trim()
      .split(/ +/g);

    if (!cmd) { return; }

    const given: Variables = {
      type: "Old",
      client,
      guild_id: message.guild.id,
      message,
      args,
    };

    const command = client.commands.get(cmd.toLowerCase()) || client.aliases.get(cmd.toLowerCase());
    if (command) { console.log(`Command: ${command.name}`); command.run(given); }
    else { message.reply("We do not have that command! :angry:") }
  }
};
