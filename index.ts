const NylonBin = require('./nylon-node.js') as {
  listen: (
    port: number,
    host: string,
    callback: () => void,
    routes: {
      [key: string]: any[]
    },
  ) => Promise<void>
  info: (message: string) => void
  debug: (message: string) => void
}

enum Method {
  Get = 'GET',
  Post = 'POST',
  Put = 'PUT',
  Delete = 'DELETE',
  Patch = 'PATCH',
  Head = 'HEAD',
  Options = 'OPTION',
  Trace = 'TRACE',
  Connect = 'CONNECT',
}
type SafeAny = any
export interface Context {
  request: Request
  response: Response
}
type Handler = (ctx: Context) => Promise<{
  is_end?: boolean
  status?: number
  headers?: {
    [key: string]: string
  }
  body?: SafeAny
}>

export interface Request {
  headers: {
    host: string
    cookie: string
    authorization: string
    'content-type': string
    'user-agent': string
    'content-length': string
    accept: string
    [key: string]: string
  }
  params: {
    [key: string]: string
  }
  method: Method
  path: string
  query: {
    [key: string]: string
  }
  body: SafeAny
}
export interface Response {
  is_end?: boolean
  status?: number // default 200
  body?: SafeAny
  headers: {
    [key: string]: string
  }
}

export const Logger = {
  info(...args: SafeAny[]) {
    return NylonBin.info(args.join(' '))
  },
  debug(...args: SafeAny[]) {
    return NylonBin.debug(args.join(' '))
  },
}

export class Nylon {
  private routes: {
    [key: string]: SafeAny[]
  } = {}

  listen(port: number, host: string, callback: () => void) {
    return NylonBin.listen(port, host, callback, this.routes)
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
