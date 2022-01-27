import { Command } from "../../Interfaces";

export const command: Command = {
  name: "help",
  aliases: ["h"],
  run: async (client, message, args) => {
    message.reply("Pong! :stuck_out_tongue_winking_eye:");
  }
}