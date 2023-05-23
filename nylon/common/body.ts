import { argsMetadataKey } from './args'

export function Body(is_raw = false) {
  return function (target: any, propertyKey: string, parameterIndex: number) {
    const args = Reflect.getOwnMetadata(argsMetadataKey, target, propertyKey) ?? []
    args[parameterIndex] = {
      type: 'body',
      value: is_raw
    }
    Reflect.defineMetadata(argsMetadataKey, args, target, propertyKey)
  }
}
