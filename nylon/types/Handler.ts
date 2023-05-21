import { Context } from './Context'
import { SafeAny } from './SafeAny'

export type Handler = (ctx: Context) => Promise<{
  is_end?: boolean
  status?: number
  headers?: {
    [key: string]: string
  }
  body?: SafeAny
}>
