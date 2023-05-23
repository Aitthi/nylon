export const MODULE_METADATA = {
  IMPORTS: 'imports',
  PROVIDERS: 'providers',
  CONTROLLERS: 'controllers',
  EXPORTS: 'exports',
}

export const INVALID_MODULE_CONFIG_MESSAGE = (_: TemplateStringsArray, property: string) =>
  `Invalid property '${property}' passed into the @Module() decorator.`

const metadataKeys = [
  MODULE_METADATA.IMPORTS,
  MODULE_METADATA.EXPORTS,
  MODULE_METADATA.CONTROLLERS,
  MODULE_METADATA.PROVIDERS,
]

export function validateModuleKeys(keys: string[]) {
  const validateKey = (key: string) => {
    if (metadataKeys.includes(key)) {
      return
    }
    throw new Error(INVALID_MODULE_CONFIG_MESSAGE`${key}`)
  }
  keys.forEach(validateKey)
}
