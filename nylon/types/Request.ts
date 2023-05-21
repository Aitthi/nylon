import { Method } from './Method'
import { SafeAny } from './SafeAny'

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
