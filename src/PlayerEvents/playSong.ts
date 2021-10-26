import { PlayerEvent } from "../Interfaces";
import { Queue, Song } from "distube";

export const event: PlayerEvent = {
  name: "playSong",
  run: (queue: Queue, song: Song) => {
    const channel = queue.textChannel;
    if (channel) {
      channel.send(
        `Playing \`${song.name}\` - \`${song.formattedDuration}\`\nRequested by: ${song.user}`);
    }
  }
}
