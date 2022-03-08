import { Command } from "../../Interfaces";

export const command: Command = {
  name: "ping",
  description: "Check if bot is online.",
  aliases: [],
  run: async (variables) => {
    const main = variables.type === "Old" ? variables.message
               : variables.interaction;

    main.reply("Pong! :stuck_out_tongue_winking_eye:");
  }
}
