import { PlayerEvent } from "../Interfaces";

export const event: PlayerEvent = {
  name: "addSong",
  run: (queue: any/*Queue*/, song: any/*Song*/) => {
    const channel = queue.textChannel;
    if (channel) {
      channel.send(
        `Added ${song.name} - \`${song.formattedDuration}\` to the queue by ${song.user}.`);
    }
  }
}
