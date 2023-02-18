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
        buffer: ArrayBuffer
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
    header: (key: string, value: string) => void
}
export class Nylon {

    private routes: {
        [key: string]: (req: Request, data_to_rs: any, handler_index: number) => Promise<SafeAny>
    } = {}

    constructor() { }

    listen(port: number, host: string, callback: Function) {
        return NylonBin.listen(port, host, callback, this.routes)
    }

    private delegate(path: string, method: Method, handlers: Handler[]) {
        let path_name = `/${method}${path ? path : '/'}`
        let handler = async (req: SafeAny, data_to_rs: any, handler_index = 0) => {
            if (!data_to_rs) data_to_rs = {
                is_json: false,
                json: null as SafeAny,
                data: null as SafeAny,
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
                }
            }
            let is_next = false
            await handlers[handler_index || 0](req, res, () => is_next = true)
            if (is_next) {
                await handler(req,data_to_rs,handler_index += 1)
            }
            return data_to_rs
        }
        this.routes[path_name] = handler
    }

    get(path: string, ...handler: Handler[]) {
        this.delegate(path, Method.Get, handler)
    }

}