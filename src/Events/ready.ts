import chalk from "chalk";
import { Event } from "../Interfaces";

export const event: Event = {
  name: "ready",
  run: (client) => {
    client.logger.log(`${chalk.magenta(client.user!.username)} is online!`);
    client.register_servers();
    client.register_yt_cookie();
    client.register_sp_tokens();
    client.register_commands();
  }
};
