import { HttpStatusCode } from './HttpStatusCode'
import { SafeAny } from './SafeAny'

export interface Response {
  is_end?: boolean
  status?: HttpStatusCode // default 200
  body?: SafeAny
  headers: {
    [key: string]: string
  }
}
