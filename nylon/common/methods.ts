function delegate(method: string) {
  return function (path?: string) {
    return function (target: any, _: string, descriptor: PropertyDescriptor) {
      let methods = Reflect.getMetadata('methods', target)
      if (!methods) {
        methods = []
      }
      path = `${method}/${path ?? ''}`
      methods.push({
        method,
        path,
        descriptor,
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
