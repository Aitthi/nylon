import { Nylon, Logger, Level, Request, Response, HttpException } from '../index'
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
          user_agent: req.header('user-agent')
        }
      })
      return res.next()
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
      res.json({
        data: {
          method: req.method(),
          name: 'Nylon',
          version: '1.0.0',
          user_agent: req.header('user-agent')
        }
      })
      return res.next()
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
