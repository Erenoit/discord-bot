import { ApplicationCommandOptionChoiceData,
         ApplicationCommandOptionType } from "discord.js";
import { Command } from "../../Interfaces";

export const command: Command = {
  name: "repeat",
  description: "Changes repeat option",
  category: "Music",
  options: [{
    name: "option",
    description: "'None', 'one' or 'all'",
    type: ApplicationCommandOptionType.String,
    required: false,
    choices: [
      {name: "None", value: "none"},
      {name: "One", value: "one"},
      {name: "All", value: "all"},
    ] as ApplicationCommandOptionChoiceData<string>[],
    //autocomplete: true,
  }],
  aliases: ["r"],
  run: (variables) => {
    variables.client.servers.get(variables.guild_id)?.player.repeat(variables);
  }
}
