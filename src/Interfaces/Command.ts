import Client from "../Client/";
import { ApplicationCommandOptionData,
         CommandInteraction, Message } from "discord.js";

interface Run {
  (variables: Old | New): void;
}

interface CommonVars {
  client: Client,
}

interface Old extends CommonVars {
  type: "Old",
  message: Message,
  args: string[],
}
interface New extends CommonVars {
  type: "New",
  interaction: CommandInteraction,
}

export type Variables = New | Old;

export type Main = Message | CommandInteraction;

export interface Command {
  name: string;
  description: string;
  options?: ApplicationCommandOptionData[],
  aliases?: string[];
  run: Run;
}
