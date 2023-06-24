import { argsMetadataKey } from './args'

export function Res() {
  return function (target: any, propertyKey: string, parameterIndex: number) {
    const args = Reflect.getOwnMetadata(argsMetadataKey, target, propertyKey) ?? []
    args[parameterIndex] = {
      type: 'response'
    }
    Reflect.defineMetadata(argsMetadataKey, args, target, propertyKey)
  }
}
