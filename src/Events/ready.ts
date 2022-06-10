import { Event } from "../Interfaces";

export const event: Event = {
  name: "ready",
  run: (client) => {
    console.log(`${client.user!} is online!`);
    client.register_servers();
    client.register_yt_cookie();
    client.register_sp_tokens();
    client.register_commands();
  }
};
