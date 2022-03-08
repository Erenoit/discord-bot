import Client from "../Client/";
import { ApplicationCommandOptionData, Message } from "discord.js";

interface Run {
  (client: Client, message: Message, args: string[]): void;
}

export interface Command {
  name: string;
  description: string;
  options?: ApplicationCommandOptionData[],
  aliases?: string[];
  run: Run;
}
