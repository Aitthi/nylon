import { validateModuleKeys } from '../utils/validate-module-keys'

export function Module(metadata: {
  imports?: any[]
  providers?: any[]
  controllers?: any[]
  exports?: any[]
}): ClassDecorator {
  const propsKeys = Object.keys(metadata as any)
  validateModuleKeys(propsKeys)
  return (target: any) => {
    for (const property in metadata) {
      Reflect.defineMetadata(property, (metadata as any)[property], target)
    }
  }
}
