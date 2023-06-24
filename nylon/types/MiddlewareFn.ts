import { Request } from './Request'
import { Response } from './Response'

export type MiddlewareFn = (
  request: Request,
  response: Response
) => Promise<{
  request: Request
  response: Response
}>
