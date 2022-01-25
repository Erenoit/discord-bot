import { PlayerEvent } from "../Interfaces";

export const event: PlayerEvent = {
  name: "playSong",
  run: (queue: any/*Queue*/, song: any/*Song*/) => {
    const channel = queue.textChannel;
    if (channel) {
      channel.send(
        `Playing \`${song.name}\` - \`${song.formattedDuration}\`\nRequested by: ${song.user}`);
    }
  }
}
