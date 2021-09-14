import "./static";
import ComputeWorker from "./compute.worker";
import { ComputeMessage } from "./custom-worker";
import { WorkerPool } from "./worker-pool";

const width = 400;
const aspectRatio = 3 / 2;
const height = width / aspectRatio;

const canvas = document.createElement("canvas");
canvas.id = "render";
canvas.width = width;
canvas.height = height;
document.body.appendChild(canvas);

const ctx = canvas.getContext("2d");

WorkerPool.create(ComputeWorker, (message: ComputeMessage) => {
    switch (message.type) {
        case "image":
            ctx.putImageData(
                message.data,
                0,
                height - message.rows - message.row0
            );
            break;
    }
}).then((pool) => {
    for (let i = 0; i < pool.size; i++) {
        pool.postMessage({
            type: "test",
            width,
            height,
            row0: Math.floor((height / pool.size) * i),
            rows: Math.floor(height / pool.size),
        });
    }
});
