export function Controller(path?: string): ClassDecorator {
  const defaultPath = '/'
  return (target: any) => {
    Reflect.defineMetadata('path', path ?? defaultPath, target)
  }
}
