import { Command } from "../../Interfaces";

export const command: Command = {
  name: "shuffle",
  description: "Adds song to queue",
  category: "Music",
  aliases: [],
  run: (variables) => {
    const plyr = variables.client.servers.get(variables.guild_id)?.player;

    if (plyr)
      variables.client.messager.send_confirm(variables,
                plyr.shuffle, plyr, [variables], "You cannot undo shuffleing.");
  }
}
