import { argsMetadataKey } from './args'

export function Req() {
  return function (target: any, propertyKey: string, parameterIndex: number) {
    const args = Reflect.getOwnMetadata(argsMetadataKey, target, propertyKey) ?? []
    args[parameterIndex] = {
      type: 'request'
    }
    Reflect.defineMetadata(argsMetadataKey, args, target, propertyKey)
  }
}
