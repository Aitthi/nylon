import * as os from 'os'
import { getHeapStatistics } from 'v8'
import { Nylon, Logger } from './../index'

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
    Logger.info('Worker', process.pid + ' is alive!')
    Logger.info('HOST_NAME', os.hostname())
    Logger.info('Platform', os.platform())
    Logger.info('Node Heap size limit', `${getHeapStatistics().heap_size_limit / (1024 * 1024)} Mb`)
    Logger.info(`🚀 Application is running on: 0.0.0.0:3000`)
  })
}

main().finally(() => {})
