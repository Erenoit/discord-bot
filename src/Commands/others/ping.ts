import { Command } from "../../Interfaces";

export const command: Command = {
  name: "ping",
  aliases: [],
  run: async (client, message, args) => {
    message.reply("Pong! :stuck_out_tongue_winking_eye:");
  }
}
