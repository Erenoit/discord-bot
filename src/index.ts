import { GatewayIntentBits } from "discord.js";
import Client from "./Client";

const intents: GatewayIntentBits = 32767;
const bot = new Client({
  intents,
  rest: { offset: 0 }
});

bot.init();

