const NylonBin = require('./../nylon-node.js') as {
  listen: (
    port: number,
    host: string,
    callback: () => void,
    routes: {
      [key: string]: any[]
    },
  ) => Promise<void>
  info: (message: string) => void
  debug: (message: string) => void
}

export const NylonNode = {
  listen: NylonBin.listen,
  logger: {
    info: NylonBin.info,
    debug: NylonBin.debug,
  },
}
