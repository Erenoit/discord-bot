import { Command } from "../../Interfaces";

export const command: Command = {
  name: "sus",
  description: "Sends sus dog",
  category: "Entertainment",
  aliases: [],
  run: async (variables) => {
    variables.client.messager.send_files(variables, "SUS!", ["./images/imposter_dog.jpg"]);
  }
};
