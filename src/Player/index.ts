import DisTube from "distube";
import Spotify from "@distube/spotify";
import SoundCloud from "@distube/soundcloud";
import { Client, Message } from "discord.js";


class MyPlayer {
  private client: Client;
  private  player: DisTube;

  constructor(client: Client) {
    this.client = client;
    this.player = new DisTube(this.client, {
      searchSongs: 1,
      searchCooldown: 30,
      leaveOnEmpty: true,
      emptyCooldown: 0,
      leaveOnFinish: true,
      leaveOnStop: true,
      plugins: [new SoundCloud(), new Spotify()],
    });
  }

  public async play(message: Message, args: string[]) {
    this.player.play(message, args.join(" "));
  }

  public async stop(message: Message) {
    this.player.stop(message);
    message.channel.send('Stopped the music!');
  }

  public async pause(message: Message) {
    this.player.pause(message);
  }

  public async resume(message: Message) {
    this.player.resume(message);
  }

  public async skip(message: Message) {
    this.player.skip(message);
  }

  public async queue(message: Message) {
    const queue = this.player.getQueue(message);
		if (!queue) { message.channel.send('Nothing playing right now!'); } 
    else {
			message.channel.send(
				`Current queue:\n${queue.songs
					.map(
						(song, id) =>
							`**${id ? id : 'Playing'}**. ${song.name} - \`${
								song.formattedDuration
							}\``,
					)
					.slice(0, 10)
					.join('\n')}`,
			);
		}
  }
}

export default MyPlayer;
