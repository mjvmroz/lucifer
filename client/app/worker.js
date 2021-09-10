import("../../compute/pkg").then((wasm) => {
    wasm.init();
    console.log(wasm.add(1, 2));
    self.addEventListener("message", (ev) => {
        try {
            // const { coords, maxIterations, exponent, tileSize } = ev.data;
            // const data = wasm.get_tile(
            //     coords.x,
            //     coords.y,
            //     coords.z,
            //     maxIterations,
            //     exponent,
            //     tileSize
            // );
            // self.postMessage({ coords: stringify(coords), pixels: data });
        } catch (err) {
            console.error(err);
        }
    });
    self.postMessage({ ready: true });
});

// TODO: better events, look into shared memory:
// https://blog.scottlogic.com/2019/07/15/multithreaded-webassembly.html
