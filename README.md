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

```ts
import * as os from 'os'
import { getHeapStatistics } from 'v8'
import { Nylon, Logger } from 'nylon-rs'

async function main() {
  const app = new Nylon()
  
  app.get('/', async () => {
    return {
      headers: {
        'Content-Type': 'text/plain',
      },
      body: `Hello, World!`,
    }
  })

  app.get(
    '/hello/:name',
    async (ctx) => {
      ctx.response.headers['Content-Type'] = 'application/json'
      if (ctx.request.params.name === 'hello') {
        return {
          ...ctx.response,
          is_end: true,
          body: {
            state_message: 'end from middleware',
            message: `Hello, ${ctx.request.params.name}`,
          },
        }
      }
      return ctx.response
    },
    async (ctx) => {
      console.info('req', ctx.request)
      console.info('res', ctx.response)
      return {
        ...ctx.response,
        body: {
          message: `Hello, ${ctx.request.params.name}`,
        },
      }
    },
  )

  await app.listen(3000, '0.0.0.0', () => {
    Logger.info(process.pid + ' is alive!', 'Worker')
    Logger.info('HOST_NAME', os.hostname())
    Logger.info('Platform', os.platform())
    Logger.info('Node Heap size limit', `${getHeapStatistics().heap_size_limit / (1024 * 1024)} Mb`)
    Logger.info(`🚀 Application is running on: 0.0.0.0:3000`)
  })
}

main().finally(() => {})
```