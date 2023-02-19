const NylonBin = require('./nylon')

enum Method {
    Get = 'GET',
    Post = 'POST',
    Put = 'PUT',
    Delete = 'DELETE',
    Patch = 'PATCH',
    Head = 'HEAD',
    Options = 'OPTION',
    Trace = 'TRACE',
    Connect = 'CONNECT'
}
type SafeAny = any
type Handler = (req: Request, res: Response, next: Function) => Promise<SafeAny>
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
    },
    body: {
        buffer: Buffer
        json?: SafeAny
    }
}

export const Logger = {
    info(...args: SafeAny[]) {
        return NylonBin.info(args.join(' '))
    },
    debug(...args: SafeAny[]) {
        return NylonBin.debug(args.join(' '))
    }
}

export interface Response {
    json: (data: SafeAny) => void
    text: (data: SafeAny) => void
    html: (data: SafeAny) => void
    send: (data: SafeAny) => void
    header: (key: string, value: string) => void,
    headres:  {
        [key: string]: string
    }
}
export class Nylon {

    private routes: {
        [key: string]: (_: null, req: Request, data_to_rs: any, handler_index: number) => Promise<SafeAny>
    } = {}

    constructor() { }

    listen(port: number, host: string, callback: Function) {
        return NylonBin.listen(port, host, callback, this.routes)
    }

    private delegate(path: string, method: Method, handlers: Handler[]) {
        let path_name = `/${method}${path ? path : '/'}`
        let handler = async (_: null, request: Request, data_to_rs: any, handler_index = 0) => {
            if(!handler_index) {
                // Vec<u8> to js Buffer
                request.body.buffer = Buffer.from(request.body.buffer)
            }
            if (!data_to_rs) data_to_rs = {
                is_json: false,
                json: {} as SafeAny,
                data: "" as SafeAny,
                res_code: 200,
                headres: {
                    "Content-Type": "text/plain"
                } as {
                    [key: string]: string
                }
            }
            let res: Response = {
                json(data) {
                    data_to_rs.json = data
                    data_to_rs.headres["Content-Type"] = "application/json"
                    data_to_rs.data = ""
                    data_to_rs.is_json = true
                },
                text(data) {
                    data_to_rs.data = data
                    data_to_rs.headres["Content-Type"] = "text/plain"
                    data_to_rs.json = {}
                    data_to_rs.is_json = false
                },
                html(data) {
                    data_to_rs.data = data
                    data_to_rs.headres["Content-Type"] = "text/html"
                    data_to_rs.json = {}
                    data_to_rs.is_json = false
                },
                send(data) {
                    data_to_rs.data = data
                    data_to_rs.json = {}
                    data_to_rs.is_json = false
                },
                header(key, value) {
                    data_to_rs.headres[key] = value
                },
                headres: data_to_rs.headres
            }
            let is_next = false
            await handlers[handler_index || 0](request, res, () => is_next = true)
            if (is_next) {
                await handler(_,request, data_to_rs, handler_index += 1)
            }
            return data_to_rs
        }
        this.routes[path_name] = handler
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