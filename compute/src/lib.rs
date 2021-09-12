mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    utils::init();
}

#[wasm_bindgen]
pub fn get_buffer(width: u32, height: u32) -> Vec<u8> {
    (0..width)
        .into_iter()
        .flat_map(|x| {
            (0..height).into_iter().flat_map(move |y| {
                //
                let r: u8 = ((x as f64 / (width - 1) as f64) * 255.999) as u8;
                let g: u8 = ((y as f64 / (height - 1) as f64) * 255.999) as u8;
                let b = 64u8;
                let a = 255u8;

                vec![r, g, b, a].into_iter()
            })
        })
        .collect()
}
