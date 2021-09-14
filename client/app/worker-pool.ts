import { SafeWorker } from "./custom-worker";

export type ReadyMessage = { type: "ready" };

export class WorkerWrapper<In, Out extends { type: string }> {
    readonly ready: Promise<void>;

    constructor(
        private readonly worker: SafeWorker<In>,
        private callback: (message: Out) => void
    ) {
        this.worker = worker;
        this.ready = new Promise<void>((ready) => {
            this.worker.onmessage = (
                event: MessageEvent<Out | ReadyMessage>
            ) => {
                if (event.data.type === "ready") {
                    ready();
                } else {
                    this.callback(event.data as Out);
                }
            };
        });
    }

    public postMessage(message: In): void {
        this.worker.postMessage(message);
    }

    public destroy(): void {
        this.callback = () => {
            // do nothing
        };
        this.worker.terminate();
    }
}

export class WorkerPool<
    In extends { type: string },
    Out extends { type: string }
> {
    private readonly workers: WorkerWrapper<In, Out>[];
    private i = 0;
    private constructor(
        readonly con: { new (): SafeWorker<In> },
        readonly callback: (message: Out) => void,
        public readonly size: number
    ) {
        this.workers = Array(size)
            .fill(0)
            .map(() => new WorkerWrapper<In, Out>(new con(), callback));
    }

    public static async create<
        In extends { type: string },
        Out extends { type: string }
    >(
        con: { new (): SafeWorker<In> },
        callback: (message: Out) => void,
        size: number = Math.min(navigator.hardwareConcurrency || 4, 64)
    ): Promise<WorkerPool<In, Out>> {
        const pool = new WorkerPool<In, Out>(con, callback, size);
        await Promise.all(pool.workers.map((worker) => worker.ready));
        return pool;
    }

    public postMessage(message: In): void {
        this.workers[this.i].postMessage(message);
        this.i = (this.i + 1) % this.workers.length;
    }
}
