import { APIEmbed } from "discord.js";
import { Command } from "../../Interfaces";
import fetch from "node-fetch";

export const command: Command = {
  name: "meme",
  description: "Sends random meme from r/memes",
  category: "Entertainment",
  aliases: [],
  run: async (variables) => {
    const link = "https://www.reddit.com/r/memes/random/.json";

    fetch(link).then((response) => {
      return response.json();
    }).then((response: any) => {
      const post = response[0].data.children[0].data;

      const url   = "https://www.reddit.com" + post.permalink;
      const title = post.title;
      const image = post.url_overridden_by_dest;

      const votes    = post.ups;
      const comments = post.num_comments;

      const embed: APIEmbed = {
        color: 0xe0af68,
        title,
        url,
        image: {url: image},
        footer: {text: `👍 ${votes} | 💬 ${comments}`}
      };

      variables.client.messager.send_embed(variables, embed);
    });
  }
};

