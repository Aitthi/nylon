import { NylonNode } from './../nylon.node'
import { SafeAny, TracingOptions } from './../types'

export { TracingOptions }

export const Tracing = {
  info(...args: SafeAny[]) {
    return NylonNode.logger.info(args.join(' '))
  },
  debug(...args: SafeAny[]) {
    return NylonNode.logger.debug(args.join(' '))
  },
  error(...args: SafeAny[]) {
    return NylonNode.logger.error(args.join(' '))
  },
  warn(...args: SafeAny[]) {
    return NylonNode.logger.warn(args.join(' '))
  },
  trace(...args: SafeAny[]) {
    return NylonNode.logger.trace(args.join(' '))
  },
}
