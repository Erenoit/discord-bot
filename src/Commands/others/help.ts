import { MessageEmbedOptions } from "discord.js";
import { Command } from "../../Interfaces";
import { bold, bold_italic, highlight } from "../../Messager";

export const command: Command = {
  name: "help",
  description: "Displays the help message.",
  category: "Other",
  aliases: ["h"],
  run: async (variables) => {
    let content = "";
    let last_category = "";

    variables.client.commands.sort((one, two) => {
      return one.category >= two.category ? 1 : -1;
    }).forEach((command) => {
      if (command.category !== last_category) {
        content += bold(`${command.category} Commands:`) + "\n";
        last_category = command.category;
      }

      let main = "";
      
      main += `\t${command.name}`;

      command.options?.forEach((option) => {
        main += ` ${highlight(`<${option.name}>`)}`;
      });

      main += ":";

      content += `${bold(main)} ${command.description}\n`;
    });

    variables.client.messager.send_normal(variables, "Help", content);
  }
};
