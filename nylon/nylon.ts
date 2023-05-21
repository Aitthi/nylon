import { NylonNode } from './nylon.node'
import { Method, SafeAny, Handler } from './types'

export class Nylon {
  private routes: {
    [key: string]: SafeAny[]
  } = {}

  listen(port: number, host: string, callback: () => void) {
    return NylonNode.listen(port, host, callback, this.routes)
  }

  private delegate(path: string, method: Method, handlers: Handler[]) {
    const path_name = `[${method}] ${path ? path : '/'}`
    this.routes[path_name] = handlers
  }

  get(path: string, ...handler: Handler[]) {
    this.delegate(path, Method.Get, handler)
  }

  post(path: string, ...handler: Handler[]) {
    this.delegate(path, Method.Post, handler)
  }

  put(path: string, ...handler: Handler[]) {
    this.delegate(path, Method.Put, handler)
  }

  delete(path: string, ...handler: Handler[]) {
    this.delegate(path, Method.Delete, handler)
  }

  patch(path: string, ...handler: Handler[]) {
    this.delegate(path, Method.Patch, handler)
  }

  head(path: string, ...handler: Handler[]) {
    this.delegate(path, Method.Head, handler)
  }

  options(path: string, ...handler: Handler[]) {
    this.delegate(path, Method.Options, handler)
  }

  trace(path: string, ...handler: Handler[]) {
    this.delegate(path, Method.Trace, handler)
  }

  connect(path: string, ...handler: Handler[]) {
    this.delegate(path, Method.Connect, handler)
  }

  all(path: string, ...handler: Handler[]) {
    this.delegate(path, Method.Get, handler)
    this.delegate(path, Method.Post, handler)
    this.delegate(path, Method.Put, handler)
    this.delegate(path, Method.Delete, handler)
    this.delegate(path, Method.Patch, handler)
    this.delegate(path, Method.Head, handler)
    this.delegate(path, Method.Options, handler)
    this.delegate(path, Method.Trace, handler)
    this.delegate(path, Method.Connect, handler)
  }
}
