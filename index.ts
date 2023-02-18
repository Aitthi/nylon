const NylonBin = require('./nylon');

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
        json?: any
    }
}

export const Logger = {
    info(...args: any[]) {
        return NylonBin.info(args.join(' '));
    },
    debug(...args: any[]) {
        return NylonBin.debug(args.join(' '));
    }
}

export interface Response {

}

type Handler = (req: Request, res: Response, next?: Function) => void

export class Nylon {

    private routes: {
        [key: string]: Handler[]
    } = {}

    constructor() { }

    listen(port: number, host: string, callback: Function) {
        return NylonBin.listen(port, host, callback, this.routes);
    }

    private delegate(path: string, method: Method, handler: Handler[]) {
        let path_name = `/${method}${path ? path : '/'}`;
        this.routes[path_name] = handler;
    }

    get(path: string, ...handler: Handler[] ) {
        this.delegate(path, Method.Get, handler)
    }

}