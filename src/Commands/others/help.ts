import { Command } from "../../Interfaces";

export const command: Command = {
  name: "help",
  description: "Displays the help message.",
  aliases: ["h"],
  run: async (variables) => {
    const main = variables.type === "Old" ? variables.message
               : variables.interaction;

    // TODO: make help message automaticly created
    main.reply(`
**The Bot**
**Creator**: Eren Önen
**Commands**:
  **- General:**
  **(help, h):** Prints this message
  **ping:** Can be used to test if bot is online or not

  **- Music:**
  **(play, p) <link or song name>:** Plays the song. If it is already playing adds the song to queue.
  **stop:** Stops the music
  **(skip, s):** Skips the current song
  **(queue, q):** Shows the queue
  `);
  }
};
