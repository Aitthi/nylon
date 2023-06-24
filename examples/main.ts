import * as os from 'os'
import { getHeapStatistics } from 'v8'
// // nylon-rs
import { NylonFactory } from '../nylon/core'
import { Module, Controller, Get, Params, Query, Post, Body, Middleware, Req, Res } from '../nylon/common'
import { Tracing, TracingOptions } from '../nylon/tracing'
import { HttpStatusCode, Request, Response } from '../nylon/types'

const middleware = async (request: Request, response: Response) => {
  request.headers['mid-name'] = 'my middleware'
  if (request.query?.state == 'end') {
    response.body = {
      end: 'my middleware'
    }
    response.is_end = true
    response.status = HttpStatusCode.Created
  }
  // console.log('[Middleware]', request, response)
  return {
    request,
    response
  }
}

@Controller()
export class MainController {
  @Get()
  index() {
    return {
      message: 'Hello World!'
    }
  }

  @Get('google')
  toGoogle(@Res() res: Response) {
    res.status = HttpStatusCode.MovedPermanently
    res.headers['Location'] = 'https://google.com'
  }

  @Post()
  @Middleware(middleware)
  indexPost(
    @Body()
    body: {
      name: string
    },
    @Req() req: Request
  ) {
    return {
      middleware_name: req.headers['mid-name'],
      message: `Hello ${body.name}!`
    }
  }

  @Post('raw')
  rawPost(
    @Body({
      raw: true
    })
    body: Buffer
  ) {
    let data = JSON.parse(body.toString()) as {
      name: string
    }
    return {
      message: `Hello ${data.name}!`
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
