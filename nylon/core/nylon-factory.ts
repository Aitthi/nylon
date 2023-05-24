import { NylonBin } from '../nylon.node'
import { Handler, MethodMetadata, NylonOptions, Request, SafeAny } from '../types'

export class NylonFactoryStatic {
  nylonBin: any
  private routes: {
    [key: string]: Handler
  } = {}

  create<T>(module: T, options: NylonOptions = {}) {
    if (!options) options = {}
    // console.info('NylonFactoryStatic.create', module)
    if (options?.tracing) {
      // TODO: Set RUST_LOG
      NylonBin.setEnv('RUST_LOG', options.tracing.join(','))
    }
    this.nylonBin = NylonBin.Nylon.init()
    this.loadModule(module)
    return {
      listen: this.listen.bind(this)
    }
  }

  private loadModule<T>(module: T) {
    let controllers: SafeAny[] = Reflect.getMetadata('controllers', module as SafeAny)
    if (!controllers) {
      controllers = []
    }
    this.registerController(controllers)
    const modules: SafeAny[] = Reflect.getMetadata('modules', module as SafeAny) ?? []
    modules.forEach((class_module: SafeAny) => this.loadModule(class_module))
  }

  private registerController(controllerCls: SafeAny[]) {
    controllerCls.forEach((class_controller: SafeAny) => {
      const controller = new class_controller()
      const basePath = Reflect.getMetadata('path', class_controller) as string
      let methods = Reflect.getMetadata('methods', controller) as MethodMetadata[]
      if (!methods) methods = []
      this.registerMethod(methods, controller, basePath)
    })
  }

  private registerMethod(methods: MethodMetadata[], controller: SafeAny, basePath: string) {
    methods.forEach((method) => {
      const handler = async (req: Request) => {
        const args = [] as SafeAny[]
        const instance = method.descriptor.value.bind(controller)
        if (method.args) {
          method.args.forEach((arg) => {
            if (arg.type === 'params') {
              args.push(req.params[arg.value])
            } else if (arg.type === 'query') {
              args.push(req.query[arg.value])
            } else if (arg.type === 'body') {
              if (arg.value) {
                args.push(req.raw_body)
              } else {
                args.push(req.body)
              }
            }
          })
        }
        const rs = await instance(...args)
        return {
          ...method.response,
          body: rs
        }
      }
      this.routes[`${basePath}${method.method}${method.path}`] = handler
    })
  }

  listen(port: number, host: string, callback: () => void) {
    this.nylonBin.addRoute(this.routes)
    return this.nylonBin.http(port, host, callback)
  }
}

export const NylonFactory: NylonFactoryStatic = new NylonFactoryStatic()
