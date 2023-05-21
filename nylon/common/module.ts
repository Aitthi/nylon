export function Module(metadata: any): ClassDecorator {
  return (target: any) => {
    for (const property in metadata) {
      Reflect.defineMetadata(property, (metadata as any)[property], target)
    }
  }
}
