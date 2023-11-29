import { Nylon, Logger, Level, Request } from '../index'
import { getHeapStatistics } from 'v8'
import os from 'os'

async function bootstrap() {
  let logger = new Logger(Level.Info)
  let app = new Nylon()

  // app register
  app.get('/', async (ctx) => {
    // console.log(ctx)
    // throw new Error(HttpException(400, 'Bad Request'))

    let req = new Request(ctx)

    return {
      data: 'Hello World!'
    }
  })

  app.post('/', async (ctx) => {
    // console.log(ctx)
    // throw new Error(HttpException(400, 'Bad Request'))
    let req = new Request(ctx)

    return {
      data: req.json()
    }
  })

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
