import "./static";

interface WorkerContainer {
    worker: Worker;
    activeJobs: number;
    ready: boolean;
}
interface AppConfig {
    workers: number;
}
let workers: Array<WorkerContainer> = [];
const config: AppConfig = {
    workers: Math.min(navigator.hardwareConcurrency || 4, 64),
};

function createWorker() {
    const w: WorkerContainer = {
        worker: new Worker("./worker.js"),
        activeJobs: 0,
        ready: false,
    };
    const workerReadyHandler = (e: MessageEvent) => {
        if (e.data.ready) {
            w.ready = true;
            w.worker.removeEventListener("message", workerReadyHandler);
        }
    };
    w.worker.addEventListener("message", workerReadyHandler);
    return w;
}

async function resetWorkers() {
    workers.forEach(({ worker }) => worker.terminate()); // terminate old workers/jobs
    workers = [...Array(config.workers)].map(createWorker);
    while (!workers.every((w) => w.ready))
        await new Promise((resolve) => setTimeout(resolve, 300));
}

function render() {
    console.log("todo");
}

resetWorkers().then(render);
