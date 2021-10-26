import { PlayerEvent } from "../Interfaces";
import { Queue, Song } from "distube";

export const event: PlayerEvent = {
  name: "addSong",
  run: (queue: Queue, song: Song) => {
    const channel = queue.textChannel;
    if (channel) {
      channel.send(
        `Added ${song.name} - \`${song.formattedDuration}\` to the queue by ${song.user}.`);
    }
  }
}
