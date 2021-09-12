mod geom;
mod ray;
mod utils;
mod vec3;

use ray::Ray;
use vec3::{Point3, Vec3};
use wasm_bindgen::prelude::*;

use crate::geom::{Hittable, Sphere};
use crate::vec3::Color;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    utils::init();
}

fn ray_color(r: &Ray) -> Color {
    if Sphere::new(Point3::new(Vec3::new(0.0, 0.0, -1.0)), 0.5).hits(r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

#[wasm_bindgen]
pub fn get_buffer(width: u32, height: u32) -> Vec<u8> {
    let aspect_ratio: f64 = width as f64 / height as f64;

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = Point3::new(
        origin.vec - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length),
    );

    (0..height)
        .rev()
        .into_iter()
        .flat_map(|y| {
            (0..width).rev().into_iter().flat_map(move |x| {
                let u = x as f64 / (width - 1) as f64;
                let v = y as f64 / (height - 1) as f64;
                let r = Ray::new(
                    origin,
                    lower_left_corner.vec + horizontal * u + vertical * v - origin.vec,
                );
                ray_color(&r).to_bytes()
            })
        })
        .collect()
}
