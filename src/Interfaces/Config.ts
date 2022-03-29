export interface SpotifyConfig {
  client_id: string,
  client_secret: string,
  refresh_token: string
}

export interface Config {
  token: string,
  prefix: string,
  yt_cookie?: string,
  spotify?: SpotifyConfig
}
