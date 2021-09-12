mod utils;
mod vec3;

use wasm_bindgen::prelude::*;

use crate::vec3::Color;

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
                Color::new(
                    x as f64 / (width - 1) as f64,
                    y as f64 / (height - 1) as f64,
                    0.25,
                )
                .to_rgba()
            })
        })
        .collect()
}
