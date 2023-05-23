import { existsSync } from 'fs'
import { join } from 'path'
import 'reflect-metadata'
import { Handler } from './types'

const localFileExisted = existsSync(join(__dirname, './../nylon-node.js'))
const localFile = localFileExisted ? './../nylon-node.js' : './nylon-node.js'
export const NylonBin = require(localFile) as {
  Logger: {
    init: () => {
      info: (message: string, scope: string) => void
      debug: (message: string, scope: string) => void
      error: (message: string, scope: string) => void
      warn: (message: string, scope: string) => void
      trace: (message: string, scope: string) => void
    }
  }
  Nylon: {
    init: () => {
      http: (port: number, host: string, callback: () => void) => Promise<boolean>
      addRoute: (routes: { [key: string]: Handler[] }) => boolean
    }
  }
  setEnv: (key: string, value: string) => void
}
