import("../../compute/pkg").then((wasm) => {
    wasm.init();
    self.addEventListener("message", (ev) => {
        if (ev.data.type === "test") {
            const imageData = new ImageData(
                new Uint8ClampedArray(
                    wasm.get_buffer(ev.data.width, ev.data.height).buffer
                ),
                ev.data.width,
                ev.data.height
            );
            self.postMessage({
                type: "image",
                data: imageData,
            });
        }
    });
    self.postMessage({
        type: "ready",
    });
});

// TODO: better events, look into shared memory:
// https://blog.scottlogic.com/2019/07/15/multithreaded-webassembly.html
