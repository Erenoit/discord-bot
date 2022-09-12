import { GatewayIntentBits } from "discord.js";
import Client from "./Client";

const intents = GatewayIntentBits.Guilds
              | GatewayIntentBits.GuildMembers
              | GatewayIntentBits.GuildBans
              | GatewayIntentBits.GuildEmojisAndStickers
              | GatewayIntentBits.GuildIntegrations
              | GatewayIntentBits.GuildWebhooks
              | GatewayIntentBits.GuildInvites
              | GatewayIntentBits.GuildVoiceStates
              | GatewayIntentBits.GuildPresences
              | GatewayIntentBits.GuildMessages
              | GatewayIntentBits.GuildMessageReactions
              | GatewayIntentBits.GuildMessageTyping
              | GatewayIntentBits.DirectMessages
              | GatewayIntentBits.DirectMessageReactions
              | GatewayIntentBits.DirectMessageTyping
              | GatewayIntentBits.MessageContent
              | GatewayIntentBits.GuildScheduledEvents;

const bot = new Client({
  intents,
  rest: { offset: 0 }
});

bot.init();

