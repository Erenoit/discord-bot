import { Command } from "../../Interfaces";

export const command: Command = {
  name: "sus",
  description: "Sends sus dog",
  aliases: [],
  run: async (variables) => {
    const main = variables.type === "Old" ? variables.message
               : variables.interaction;

    main.reply({
      content: "SUS!",
      files: ["./images/imposter_dog.jpg"]
    });
  }
};
