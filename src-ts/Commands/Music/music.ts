import { ApplicationCommandOptionType } from "discord.js";
import { Command } from "../../Interfaces";

export const command: Command = {
  name: "music",
  description: "Shortcuts for common music links",
  category: "Music",
  options: [{
    name: "key",
    description: "Key of wanted list/video",
    type: ApplicationCommandOptionType.String,
    required: true
  }],
  aliases: ["m"],
  run: (variables) => {
    // TODO: use database to store key-url pairs
    const keywords = new Map([
      ["gachi",      "https://www.youtube.com/watch?v=vysM33WCieE"],
      ["trump",      "https://www.youtube.com/watch?v=y5ki_VGlmiM"],
      ["pirate",     "https://www.youtube.com/watch?v=iKJlhhf_lhs"],
      ["chillexy",   "https://open.spotify.com/playlist/7sTDKMewsaANlRSGUQVzPU?si=TeOQQjUeTimMw6Jd9mVEFQ"],
      ["can_fav",    "https://www.youtube.com/playlist?list=PLqbnwol7YwR4SKldGdb1-43rZB6PljLe7"],
      ["svetlana",   "https://open.spotify.com/playlist/6gHJvb4rlKdUNW4Q9DjXRw?si=DWU8slGTQ9yeyr0uDmu0RA"],
      ["kpop1",      "https://www.youtube.com/watch?v=citgluw97m8"],
      ["kpop2",      "https://www.youtube.com/watch?v=18nDrsoii5M&list=RDCLAK5uy_mHW5bcduhjB-PkTePAe6EoRMj1xNT8gzY&start_radio=1"],
      ["pentakill3", "https://www.youtube.com/watch?v=VXtaMAN9zX4"],
      ["songul",     "https://www.youtube.com/watch?v=hIiAJ69o3Zw"],
      ["anime1",     "https://www.youtube.com/playlist?list=PLqbnwol7YwR7WGvjEDjjd9GeLx-KRmipa"],
      ["anime2",     "https://open.spotify.com/playlist/49ZreaOgQ0dMirgecTPh0n?si=8f88241e9a7b4a01"],
      ["anime3",     "https://www.youtube.com/watch?v=Mn7Bv8rGRzg"],
      ["anime4",     "https://www.youtube.com/watch?v=J0S6tc6dIK8"],
      ["yuki",       "https://www.youtube.com/watch?v=KO-G5DVNlw4"],
      ["ayaya",      "https://www.youtube.com/watch?v=9wnNW4HyDtg"],
    ]);

    try {
      let args = variables.type === "Old"
               ? variables.args.join("").trim()
               : variables.interaction.options.getString("key")!;

      if (args === "help") {
        variables.client.messager.send_err(variables,
                  `Usable keys:\n\t${Array.from(keywords.keys()).join(", ")}.`);
        return;
      }

      const url = keywords.get(args);

      if (url === undefined) {
        throw "Error";
      }

      variables.client.servers.get(variables.guild_id)?.player.play(variables, url);
    } catch {
      variables.client.messager.send_err(variables,
                "Invalid argument. Use `-music help` for more information.");
      return;
    }
  }
}
