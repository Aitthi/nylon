import { NylonBin } from './../nylon.node'
import { SafeAny, TracingOptions } from './../types'

export { TracingOptions }

const TracingFunc = (spanName = '') => {
  return {
    info(...args: SafeAny[]) {
      return NylonBin.Logger.init().info(args.join(' '), spanName)
    },
    debug(...args: SafeAny[]) {
      return NylonBin.Logger.init().debug(args.join(' '), spanName)
    },
    error(...args: SafeAny[]) {
      return NylonBin.Logger.init().error(args.join(' '), spanName)
    },
    warn(...args: SafeAny[]) {
      return NylonBin.Logger.init().warn(args.join(' '), spanName)
    },
    trace(...args: SafeAny[]) {
      return NylonBin.Logger.init().trace(args.join(' '), spanName)
    }
  }
}

export const Tracing = {
  ...TracingFunc(''),
  scope(name: string) {
    return TracingFunc(name)
  }
}
