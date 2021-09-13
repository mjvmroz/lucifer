import { ComputeInstruction, ComputeMessage, SafeWorker } from "./custom-worker";
import { ReadyMessage } from "./worker-pool";

// @ts-expect-error - the irony of unsafe assertions for _extra_ safety
const ctx: SafeWorker<ComputeMessage | ReadyMessage> = self;

import("../../compute/pkg").then((wasm) => {
    wasm.init();
    ctx.addEventListener("message", (ev: MessageEvent<ComputeInstruction>) => {
        if (ev.data.type === "test") {
            console.log("Generating image data for...", ev.data);
            const imageData = new ImageData(
                new Uint8ClampedArray(
                    wasm.get_buffer(ev.data.width, ev.data.height, ev.data.row0, ev.data.rows).buffer
                ),
                ev.data.width,
                ev.data.rows
            );
            ctx.postMessage({
                type: "image",
                width: ev.data.width,
                height: ev.data.height,
                row0: ev.data.row0,
                rows: ev.data.rows,
                data: imageData,
            });
        }
    });
    ctx.postMessage({
        type: "ready",
    });
});

export default null as { new (): SafeWorker<ComputeInstruction> };
