import { MiddlewareFn } from '../types'

export function Middleware(...middlewareFn: MiddlewareFn[]) {
  return function (_: any, __: string, descriptor: PropertyDescriptor) {
    Reflect.defineMetadata('middleware', middlewareFn, descriptor.value)
  }
}
