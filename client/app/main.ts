import { ComputeMessage } from "./messages";
import "./static";

const width = 400;
const aspectRatio = 16 / 9;
const height = width / aspectRatio;

const canvas = document.createElement("canvas");
canvas.width = width;
canvas.height = height;
document.body.appendChild(canvas);

const ctx = canvas.getContext("2d");

const worker = new Worker(new URL("./compute.worker.ts", import.meta.url));

worker.onmessage = (event: MessageEvent<ComputeMessage>) => {
    const { data } = event;
    if (data.type === "ready") {
        worker.postMessage({ type: "test", width, height });
    } else if (data.type === "image") {
        ctx.putImageData(data.data, 0, 0);
    }
};