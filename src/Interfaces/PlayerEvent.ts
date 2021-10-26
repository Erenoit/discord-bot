interface Run {
  (...args: any): void
}

export interface PlayerEvent {
  name: string,
  run: Run
}
