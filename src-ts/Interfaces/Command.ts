import Client from "../Client/";
import { ApplicationCommandOptionData,
         ChatInputCommandInteraction, Message,
         Snowflake } from "discord.js";

interface Run {
  (variables: Variables): void;
}

interface CommonVars {
  client: Client,
  guild_id: Snowflake
}

interface Old extends CommonVars {
  type: "Old",
  message: Message,
  args: string[],
}
interface New extends CommonVars {
  type: "New",
  interaction: ChatInputCommandInteraction,
}

export type Variables = New | Old;

export interface Command {
  name: string,
  description: string,
  category: string,
  options?: ApplicationCommandOptionData[],
  aliases: string[],
  run: Run,
}
