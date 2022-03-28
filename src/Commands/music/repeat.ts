import { ApplicationCommandOptionChoice } from "discord.js";
import { ApplicationCommandOptionTypes } from "discord.js/typings/enums";
import { Command } from "../../Interfaces";

export const command: Command = {
  name: "repeat",
  description: "Changes repeat option",
  options: [{
    name: "option",
    description: "'None', 'one' or 'all'",
    type: ApplicationCommandOptionTypes.STRING,
    required: false,
    choices: [
      {name: "None", value: "none"},
      {name: "One", value: "one"},
      {name: "All", value: "all"},
    ] as ApplicationCommandOptionChoice[],
    //autocomplete: true,
  }],
  aliases: ["r"],
  run: (variables) => {
    variables.client.player.repeat(variables);
  }
}
