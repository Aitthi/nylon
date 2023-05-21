import { existsSync } from 'fs'
import 'reflect-metadata'

const localFileExisted = existsSync('./../nylon-node.js')
const localFile = localFileExisted ? './nylon-node.js' : './../nylon-node.js'
const NylonBin = require(localFile) as {
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
