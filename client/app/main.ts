import "./static";

type WorkerMessage = { type: "ready" } | { type: "image"; data: ImageData };

const width = 800;
const aspectRatio = 16 / 9;
const height = width / aspectRatio;

const canvas = document.createElement("canvas");
canvas.width = width;
canvas.height = height;
document.body.appendChild(canvas);

const ctx = canvas.getContext("2d");

const worker = new Worker("./worker.js");
worker.addEventListener("message", (e: MessageEvent<WorkerMessage>) => {
    switch (e.data.type) {
        case "ready":
            worker.postMessage({
                type: "test",
                width,
                height,
            });
            break;
        case "image":
            ctx.putImageData(e.data.data, 0, 0);
            break;
    }
});
