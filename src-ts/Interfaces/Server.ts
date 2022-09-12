import Player from "../Player";

export interface Server {
  guild_id: string,
  prefix: string,
  player: Player,
};
