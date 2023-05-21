import { NylonNode } from './nylon.node'
import { SafeAny } from './types'

export const Logger = {
  info(...args: SafeAny[]) {
    return NylonNode.logger.info(args.join(' '))
  },
  debug(...args: SafeAny[]) {
    return NylonNode.logger.debug(args.join(' '))
  },
}
