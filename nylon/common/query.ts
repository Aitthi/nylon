import { argsMetadataKey } from './args'

export function Query(key: string) {
  return function (target: any, propertyKey: string, parameterIndex: number) {
    const args = Reflect.getOwnMetadata(argsMetadataKey, target, propertyKey) ?? []
    args[parameterIndex] = {
      type: 'query',
      value: key
    }
    Reflect.defineMetadata(argsMetadataKey, args, target, propertyKey)
  }
}
