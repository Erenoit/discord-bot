import { Event } from "../Interfaces";

export const event: Event = {
  name: "ready",
  run: (client) => {
    console.log(`${(client.user || {tag: "ERROR"}).tag} is online!`);
  }
};
