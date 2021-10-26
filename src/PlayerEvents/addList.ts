import { PlayerEvent } from "../Interfaces";
import { Playlist, Queue } from "distube";

export const event: PlayerEvent = {
  name: "addList",
  run: (queue: Queue, playlist: Playlist) => {
    const channel = queue.textChannel;
    if (channel) {
      channel.send(
        `Added \`${playlist.name}\` playlist (${playlist.songs.length} songs) to the queue!`);
    }
  }
}
