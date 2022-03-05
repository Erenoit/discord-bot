interface SpConfig {
  client_id: string,
  client_secret: string,
  refresh_token: string
}

export interface Config {
  token: string,
  yt_cookie?: string,
  spotify?: SpConfig,
  prefix: string,
}
