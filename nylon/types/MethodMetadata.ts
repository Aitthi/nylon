import { Response } from './Response'

export interface MethodMetadata {
  method: string
  path: string
  descriptor: PropertyDescriptor
  args: {
    type: 'params' | 'query' | 'body' | 'request' | 'response'
    value?: any
  }[]
  response: Response
}
