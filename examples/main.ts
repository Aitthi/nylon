import * as os from 'os'
import { getHeapStatistics } from 'v8'
// // nylon-rs
import { NylonFactory } from '../nylon/core'
import { Module, Controller, Get, Params, Query, Post, Body } from '../nylon/common'
import { Tracing, TracingOptions } from '../nylon/tracing'

@Controller()
export class MainController {
  @Get()
  index() {
    return {
      message: 'Hello World!'
    }
  }

  @Post()
  indexPost(
    @Body()
    body: {
      name: string
    }
  ) {
    return {
      message: `Hello ${body.name}!`
    }
  }

  @Get('hello/:name')
  async hello(@Params('name') name: string, @Query('age') age: number) {
    return {
      message: `Hello ${name}! You are ${age || 1} years old.`
    }
  }
}

@Module({
  controllers: [MainController]
})
export class MainModule {}

async function bootstrap() {
  const app = NylonFactory.create(MainModule, {
    tracing: [TracingOptions.DEBUG]
  })
  await app.listen(3000, '0.0.0.0', () => {
    let TracingScope = Tracing.scope('Bootstrap')
    TracingScope.info('Worker', process.pid + ' is alive!')
    TracingScope.info('HOST_NAME', os.hostname())
    TracingScope.info('Platform', os.platform())
    TracingScope.info('Node Heap size limit', `${getHeapStatistics().heap_size_limit / (1024 * 1024)} Mb`)
    TracingScope.info(`🚀 Application is running on: 0.0.0.0:3000`)
  })
}

bootstrap().finally(() => {})
