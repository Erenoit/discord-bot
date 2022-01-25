import { PlayerEvent } from "../Interfaces";

export const event: PlayerEvent = {
  name: "empty",
  run: (queue: any/*Queue*/) => {
    const channel = queue.textChannel;
    if (channel) {
      channel.send("Channel is empty. Leaving the channel");
    }
  }
}
