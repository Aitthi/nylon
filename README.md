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
import { Nylon, Logger, Level, Request, Response, HttpException } from 'nylon-rs'
import { getHeapStatistics } from 'v8'
import os from 'os'

async function bootstrap() {
  let logger = new Logger(Level.Info)
  let app = new Nylon()

  app.get('/', [
    async (ctx) => {
      let req = new Request(ctx)
      let res = new Response(ctx)
      res.json({
        data: {
          name: 'Nylon',
          version: '1.0.0',
          path: req.path(),
          query: req.queries(),
          user_agent: req.header('user-agent')
        }
      })
      return res.jump()
    },
    async (ctx) => {
      // throw new Error(HttpException(401, 'Unauthorized'))

      let res = new Response(ctx)
      res.status(201)
      return res.end()
    }
  ])

  app.get('/:name', [
    async (ctx) => {
      let req = new Request(ctx)
      let res = new Response(ctx)
      res.json({
        data: {
          is_params: true,
          name: 'Nylon',
          version: '1.0.0',
          path: req.path(),
          query: req.queries(),
          user_agent: req.header('user-agent')
        }
      })
      return res.jump()
    },
    async (ctx) => {
      // throw new Error(HttpException(401, 'Unauthorized'))

      let res = new Response(ctx)
      res.status(201)
      return res.end()
    }
  ])

  app.post('/', [
    async (ctx) => {
      let req = new Request(ctx)
      let res = new Response(ctx)
      let multipart = await req.multipart({
        limit: '5mb', // 10mb, 1kb only support kb, mb // 10 * 1024 * 1024
        allowed_fields: ['name', 'file'] // optional
      })
      console.log('multipart', multipart)
      res.json({
        data: {
          method: req.method(),
          name: 'Nylon',
          version: '1.0.0',
          user_agent: req.header('user-agent')
        }
      })
      return res.jump()
    },
    async (ctx) => {
      // throw new Error(HttpException(401, 'Unauthorized'))

      let res = new Response(ctx)
      res.status(201)
      return res.end()
    }
  ])

  await app.listen(3000, '0.0.0.0', () => {
    let scopeScope = logger.scope('Bootstrap')
    scopeScope.info(['Worker', process.pid + ' is alive!'].join(' '))
    scopeScope.info(['HOST_NAME', os.hostname()].join(' '))
    scopeScope.info(['Platform', os.platform()].join(' '))
    scopeScope.info(['Node Heap size limit', `${getHeapStatistics().heap_size_limit / (1024 * 1024)} Mb`].join(' '))
    scopeScope.info(`🚀 Application is running on: 0.0.0.0:3000`)
  })
}

// Bootstrap for bun 1.0.x
// @ts-ignore
// await bootstrap().then(() => {
//   console.log('Bootstrap done!')
// })

bootstrap().then(() => {
  console.log('Bootstrap done!')
})
```
