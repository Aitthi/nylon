import { Response } from '../types'
import { argsMetadataKey } from './args'

function delegate(method: string) {
  return function (path?: string) {
    return function (target: any, propertyName: string, descriptor: PropertyDescriptor) {
      let methods = Reflect.getMetadata('methods', target)
      if (!methods) {
        methods = []
      }
      path = `/${path ?? ''}`
      const args = Reflect.getOwnMetadata(argsMetadataKey, target, propertyName)
      const response: Response = {
        is_end: false,
        status: 200,
        headers: {
          'Content-Type': 'application/json',
        },
        body: null,
      }
      methods.push({
        method,
        path,
        descriptor,
        args,
        response,
      })
      Reflect.defineMetadata('methods', methods, target)
    }
  }
}

export function Get(path?: string) {
  return delegate('GET')(path)
}

export function Post(path?: string) {
  return delegate('POST')(path)
}

export function Put(path?: string) {
  return delegate('PUT')(path)
}

export function Delete(path?: string) {
  return delegate('DELETE')(path)
}

export function Patch(path?: string) {
  return delegate('PATCH')(path)
}

export function Options(path?: string) {
  return delegate('OPTIONS')(path)
}

export function Head(path?: string) {
  return delegate('HEAD')(path)
}

export function All(path?: string) {
  return delegate('ALL')(path)
}
