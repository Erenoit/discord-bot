import { PlayerEvent } from "../Interfaces";
import { Queue } from "distube";

export const event: PlayerEvent = {
  name: "empty",
  run: (queue: Queue) => {
    const channel = queue.textChannel;
    if (channel) {
      channel.send("Channel is empty. Leaving the channel");
    }
  }
}
