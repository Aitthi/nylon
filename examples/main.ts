import * as os from 'os'
import { getHeapStatistics } from 'v8'
// // nylon-rs
import { NylonFactory } from '../nylon/core'
import { Module, Controller, Get } from '../nylon/common'
import { Tracing, TracingOptions } from '../nylon/tracing'
import { Context } from '../nylon/types'

@Controller()
export class MainController {
  
  @Get()
  index() {
    return {
      message: 'Hello World!'
    }
  }


  @Get('hello/:name')
  async hello(ctx: Context) {
    return {
      message: `Hello ${ctx.request.params.name}!`
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
