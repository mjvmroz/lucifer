mod camera;
mod color;
mod geom;
mod math;
mod ray;
mod utils;
mod vec3;

use js_sys::Math::random;
use ray::Ray;
use vec3::{Point3, Vec3};
use wasm_bindgen::prelude::*;

use crate::color::Color;
use crate::geom::{Hittable, Scene, Sphere};
use crate::math::js_rand;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    utils::init();
}

fn ray_color(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    if depth <= 0 {
        return Color::BLACK;
    }
    scene.hit(ray, 0.001, f64::INFINITY).map_or_else(
        || {
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            Color::WHITE.blend(&Color::new(0.5, 0.7, 1.0), t)
        },
        |record| {
            let target = record.p.vec + record.normal + Vec3::random_unit_vector();
            ray_color(&Ray::new(record.p, target - record.p.vec), scene, depth - 1) * 0.5
        },
    )
}

#[wasm_bindgen]
pub fn get_buffer(width: u32, height: u32) -> Vec<u8> {
    let aspect_ratio: f64 = width as f64 / height as f64;

    // Scene
    let sphere = Sphere::new(Point3::vec(0.0, 0.0, -1.0), 0.5);
    let planet = Sphere::new(Point3::vec(0.0, -100.5, -1.0), 100.0);
    let scene = &Scene::new(vec![Box::new(sphere), Box::new(planet)]);

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::ZERO;
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = Point3::new(
        origin.vec - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length),
    );

    // Image
    let samples_per_pixel = 20;
    let max_depth = 10;

    (0..height)
        .rev()
        .into_iter()
        .flat_map(|y| {
            (0..width).into_iter().flat_map(move |x| {
                let sampled_color =
                    (0..samples_per_pixel)
                        .into_iter()
                        .fold(Color::BLACK, move |acc, _i| {
                            let u: f64 = (x as f64 + js_rand()) / (width - 1) as f64;
                            let v: f64 = (y as f64 + js_rand()) / (height - 1) as f64;
                            let r = Ray::new(
                                origin,
                                lower_left_corner.vec + horizontal * u + vertical * v - origin.vec,
                            );
                            acc + ray_color(&r, scene, max_depth)
                        });
                (sampled_color / samples_per_pixel as f64).sqrt().to_bytes()
            })
        })
        .collect()
}
