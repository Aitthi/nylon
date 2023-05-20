declare enum Method {
    Get = "GET",
    Post = "POST",
    Put = "PUT",
    Delete = "DELETE",
    Patch = "PATCH",
    Head = "HEAD",
    Options = "OPTION",
    Trace = "TRACE",
    Connect = "CONNECT"
}
type SafeAny = any;
export interface Context {
    request: Request;
    response: Response;
}
type Handler = (ctx: Context) => Promise<{
    is_end?: boolean;
    status?: number;
    headers?: {
        [key: string]: string;
    };
    body?: SafeAny;
}>;
export interface Request {
    headers: {
        host: string;
        cookie: string;
        authorization: string;
        'content-type': string;
        'user-agent': string;
        'content-length': string;
        accept: string;
        [key: string]: string;
    };
    params: {
        [key: string]: string;
    };
    method: Method;
    path: string;
    query: {
        [key: string]: string;
    };
    body: SafeAny;
}
export interface Response {
    is_end?: boolean;
    status?: number;
    body?: SafeAny;
    headers: {
        [key: string]: string;
    };
}
export declare const Logger: {
    info(...args: SafeAny[]): void;
    debug(...args: SafeAny[]): void;
};
export declare class Nylon {
    private routes;
    listen(port: number, host: string, callback: () => void): Promise<void>;
    private delegate;
    get(path: string, ...handler: Handler[]): void;
    post(path: string, ...handler: Handler[]): void;
    put(path: string, ...handler: Handler[]): void;
    delete(path: string, ...handler: Handler[]): void;
    patch(path: string, ...handler: Handler[]): void;
    head(path: string, ...handler: Handler[]): void;
    options(path: string, ...handler: Handler[]): void;
    trace(path: string, ...handler: Handler[]): void;
    connect(path: string, ...handler: Handler[]): void;
    all(path: string, ...handler: Handler[]): void;
}
export {};
