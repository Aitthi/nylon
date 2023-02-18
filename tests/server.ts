import * as os from 'os';
import { getHeapStatistics } from 'v8';
import { Nylon, Logger } from './../index'

async function main(){

    let app = new Nylon();
    let sleep = (ms: number) => new Promise((s,j)=>{
        setTimeout(()=>s(true), ms)
    })
    app.get("/", async (req, res, next) => {
        await sleep(3000)
        next()
    }, async (req, res, next) => {
        res.header('server', 'Nylon/0.1.0')
        next()
    }, async (req, res)=>{
        res.json({
            message: "TS 200 OK"
        })
    })

    await app.listen(3000, '0.0.0.0', () => {
        Logger.info(process.pid + " is alive!", "Worker");
        Logger.info("HOST_NAME", os.hostname())
        Logger.info("Platform", os.platform())
        Logger.info("Node Heap size limit", `${getHeapStatistics().heap_size_limit / (1024 * 1024)} Mb`)
        Logger.info(
          `🚀 Application is running on: 0.0.0.0:3000`
        );    
    })
    
}

main()