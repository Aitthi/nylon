import { SafeAny } from './SafeAny'

export interface Response {
  is_end?: boolean
  status?: number // default 200
  body?: SafeAny
  headers: {
    [key: string]: string
  }
}
