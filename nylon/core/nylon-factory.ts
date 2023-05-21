import { NylonNode } from '../nylon.node'
import { Context, Handler, NylonOptions, SafeAny } from '../types'

export class NylonFactoryStatic {
  private routes: {
    [key: string]: SafeAny[]
  } = {}

  create<T>(module: T, options: NylonOptions = {}) {
    if (!options) options = {}
    // console.info('NylonFactoryStatic.create', module)
    if (options?.tracing) {
      // TODO: Set RUST_LOG
    }
    this.loadModule(module)
    return {
      listen: this.listen.bind(this),
    }
  }

  private loadModule(module: any) {
    let controllers: SafeAny[] = Reflect.getMetadata('controllers', module as SafeAny)
    if (!controllers) {
      controllers = []
    }
    controllers.forEach((class_controller: SafeAny) => {
      const controller = new class_controller()
      const basePath = Reflect.getMetadata('path', class_controller) as string
      let methods = Reflect.getMetadata('methods', controller) as {
        method: string
        path: string
        descriptor: PropertyDescriptor
      }[]
      if (!methods) methods = []
      methods.forEach((method) => {
        const handlers = [] as Handler[]
        const handler = async (ctx: Context) => {
          const instance = method.descriptor.value.bind(controller)
          const rs = await instance(ctx)
          return {
            ...ctx.response,
            body: rs,
          }
        }
        handlers.push(handler)
        this.routes[`${basePath}${method.path}`] = handlers
      })
    })
    const modules: SafeAny[] = Reflect.getMetadata('modules', module as SafeAny) ?? []
    modules.forEach((class_module: SafeAny) => this.loadModule(class_module))
  }

  listen(port: number, host: string, callback: () => void) {
    return NylonNode.listen(port, host, callback, this.routes)
  }
}

export const NylonFactory: NylonFactoryStatic = new NylonFactoryStatic()