import { Command } from "../../Interfaces";

export const command: Command = {
  name: "sus",
  description: "Sends sus dog",
  aliases: [],
  run: async (client, message, args) => {
    message.channel.send({
      content: "SUS!",
      files: ["./images/imposter_dog.jpg"]
    });
  }
};
