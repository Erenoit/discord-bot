import { Interaction } from "discord.js";
import { Event, Variables } from "../Interfaces";

export const event: Event = {
  name: "interactionCreate",
  run: (client, interaction: Interaction) => {
    if (interaction.isCommand()) {
      if (interaction.user.bot) {
        interaction.reply("Bots cannot use commands!");
        return;
      }

      const guild_id = interaction.guild?.id;

      if (!guild_id) {
        interaction.reply("An error accured. Please try again.");
        return;
      }

      const cmd = client.commands.get(interaction.commandName);

      if (!cmd) {
        interaction.reply("An error accured. Please try again.");
        return;
      }

      client.logger.log(`Command: ${cmd.name}`);

      const given: Variables = {
        type: "New",
        guild_id,
        client,
        interaction
      };

      cmd.run(given);
    }
  }
};
