import { PlayerEvent } from "../Interfaces";

export const event: PlayerEvent = {
  name: "addList",
  run: (queue: any/*Queue*/, playlist: any/*Playlist*/) => {
    const channel = queue.textChannel;
    if (channel) {
      channel.send(
        `Added \`${playlist.name}\` playlist (${playlist.songs.length} songs) to the queue!`);
    }
  }
}
