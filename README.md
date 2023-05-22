# Nylon Experimental

[![NPM version](https://img.shields.io/npm/v/nylon-rs.svg?style=for-the-badge)](https://www.npmjs.com/package/nylon-rs)

Nylon is a web framework for Node.js built with Tokio, Tower, Hyper, and Napi-rs


## Installation

```bash
npm install nylon-rs
```
or
```bash
yarn add nylon-rs
```


## Usage

edit tsconfig.json
```json
{
  "compilerOptions": {
    "emitDecoratorMetadata": true,
    "experimentalDecorators": true,
    ....
  }
}
```

```ts
import * as os from 'os'
import { getHeapStatistics } from 'v8'
import { NylonFactory } from 'nylon-rs/core'
import { Module, Controller, Get, Params, Query } from 'nylon-rs/common'
import { Tracing, TracingOptions } from 'nylon-rs/tracing'

@Controller()
export class MainController {
  
  @Get()
  index() {
    return {
      message: 'Hello World!'
    }
  }


  @Get('hello/:name')
  async hello(
    @Params('name') name: string,
    @Query('age') age: number,
  ) {
    return {
      message: `Hello ${name}! You are ${age || 1} years old.`
    }
  }

}

@Module({
    controllers: [
      MainController
    ],
})
export class MainModule {}

async function bootstrap() {
  const app = NylonFactory.create(MainModule, {
    tracing: [TracingOptions.DEBUG],
  })
  await app.listen(3000, '0.0.0.0', () => {
    Tracing.info('Worker', process.pid + ' is alive!')
    Tracing.info('HOST_NAME', os.hostname())
    Tracing.info('Platform', os.platform())
    Tracing.info('Node Heap size limit', `${getHeapStatistics().heap_size_limit / (1024 * 1024)} Mb`)
    Tracing.info(`🚀 Application is running on: 0.0.0.0:3000`)
  })
}

bootstrap().finally(() => {})
```