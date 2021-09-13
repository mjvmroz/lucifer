import { ComputeInstruction } from "./messages";

const initHandlers = async () => {
    const wasm = await import("../../compute/pkg");
    await wasm.default();
    await wasm.initThreadPool(navigator.hardwareConcurrency);

    self.onmessage = (ev: MessageEvent<ComputeInstruction>) => {
        if (ev.data.type === "test") {
            console.log("Generating image data for...", ev.data);
            const imageData = new ImageData(
                new Uint8ClampedArray(
                    wasm.get_buffer(ev.data.width, ev.data.height).buffer
                ),
                ev.data.width,
                ev.data.height,
            );
            console.log("Image data generated!");
            self.postMessage({
                type: "image",
                data: imageData,
            });
        }
    };

    self.postMessage({
        type: "ready",
    });
};

initHandlers();