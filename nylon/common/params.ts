import { argsMetadataKey } from './args'

export function Params(key: string) {
  return function (target: any, propertyKey: string, parameterIndex: number) {
    const args = Reflect.getOwnMetadata(argsMetadataKey, target, propertyKey) ?? []
    args[parameterIndex] = {
      type: 'params',
      value: key,
    }
    Reflect.defineMetadata(argsMetadataKey, args, target, propertyKey)
  }
}
