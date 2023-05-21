import { Request } from './Request'
import { Response } from './Response'

export interface Context {
  request: Request
  response: Response
}
