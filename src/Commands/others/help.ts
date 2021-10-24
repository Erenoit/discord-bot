import { Command } from "../../Interfaces";

export const command: Command = {
  name: "help",
  aliases: ["h"],
  run: async (client, message, args) => {
    message.channel.send("Help is not ready yet :(");
  }
};
