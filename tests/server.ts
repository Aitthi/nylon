import * as os from 'os';
import { getHeapStatistics } from 'v8';
import { Nylon, Logger } from './../index'

async function main(){

    let app = new Nylon();

    app.get("/", (req, res)=>{
        return {
            message: "TS 200 OK",
            code: 200
        }
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