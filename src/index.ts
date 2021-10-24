import { Intents, Message } from "discord.js";
import Client from "./Client";

const intents = new Intents(32767);
const bot = new Client({ intents });

bot.init();

