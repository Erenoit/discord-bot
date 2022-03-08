import { Interaction } from "discord.js";
import { Event, Variables } from "../Interfaces";

export const event: Event = {
  name: "interactionCreate",
  run: (client, interaction: Interaction) => {
    //console.log(interaction);
    if (interaction.isCommand()) {
      if (interaction.user.bot) {
        interaction.reply("Bots cannot use commands!");
        return;
      }

      const cmd = client.commands.get(interaction.commandName);

      if (!cmd) {
        interaction.reply("An error accured. Please try again.");
        return;
      }

      console.log(`Command: ${cmd.name}`);

      const given: Variables = {
        type: "New",
        client,
        interaction
      }

      cmd.run(given);

    }
  }
};
