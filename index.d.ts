export function info(log: string): void
export function debug(log: string): void
export function listen(
  port: number,
  host: string,
  callback: (err: Error | null, value: undefined) => any,
  routes: Record<string, Array<Handler>>,
): Promise<boolean>
