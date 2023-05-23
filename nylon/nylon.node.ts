import { existsSync } from 'fs'
import { join } from 'path'
import 'reflect-metadata'

const localFileExisted = existsSync(join(__dirname, './../nylon-node.js'))
const localFile = localFileExisted ? './../nylon-node.js' : './nylon-node.js'
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
  error: (message: string) => void
  warn: (message: string) => void
  trace: (message: string) => void
  setEnv: (key: string, value: string) => void
}

// console.info('NylonBin', NylonBin)

export const NylonNode = {
  listen: NylonBin.listen,
  logger: {
    info: NylonBin.info,
    debug: NylonBin.debug,
    error: NylonBin.error,
    warn: NylonBin.warn,
    trace: NylonBin.trace,
  },
  set_env: NylonBin.setEnv,
}
