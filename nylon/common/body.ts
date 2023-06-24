import { BodyOptions } from '../types/BodyOptions'
import { argsMetadataKey } from './args'

export function Body(options = {} as BodyOptions) {
  options = {
    ...{
      raw: false
    },
    ...options
  }
  return function (target: any, propertyKey: string, parameterIndex: number) {
    const args = Reflect.getOwnMetadata(argsMetadataKey, target, propertyKey) ?? []
    args[parameterIndex] = {
      type: 'body',
      value: options
    }
    Reflect.defineMetadata(argsMetadataKey, args, target, propertyKey)
  }
}
