import { Request } from './Request'
import { SafeAny } from './SafeAny'

export type Handler = (req: Request) => Promise<{
  is_end?: boolean
  status?: number
  headers?: {
    [key: string]: string
  }
  body?: SafeAny
}>
