mod camera;
mod color;
mod geom;
mod material;
mod math;
mod ray;
mod utils;
mod vec3;

use ray::Ray;
use vec3::{Point3, Vec3};
use wasm_bindgen::prelude::*;

use crate::color::Color;
use crate::geom::{Hittable, Scene, Sphere};
use crate::material::Material;

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
        Color::BLACK
    } else {
        scene.hit(ray, 0.001, f64::INFINITY).map_or_else(
            || {
                let unit_direction = ray.direction.unit_vector();
                let t = 0.5 * (unit_direction.y + 1.0);
                Color::WHITE.blend(&Color::new(0.5, 0.7, 1.0), t)
            },
            |record| {
                record.material.scatter(ray, &record).map_or_else(|| Color::BLACK, |scatter_record| {
                    scatter_record.attenuation * ray_color(&scatter_record.scattered, scene, depth - 1)
                })
            },
        )
    }
}

#[wasm_bindgen]
pub fn get_buffer(
    width: u32,
    height: u32,
    row0: u32,
    rows: u32,
) -> Vec<u8> {
    let aspect_ratio: f64 = width as f64 / height as f64;

    // Scene
    let material_ground = Material::Lambertian { albedo: Color::new(0.8, 0.8, 0.0) };
    let material_left = Material::Metal { albedo: Color::new(0.8, 0.8, 0.8), fuzz: 0.3 };
    let material_center = Material::Lambertian { albedo: Color::new(0.7, 0.3, 0.3) };
    let material_right = Material::Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 1.0 };

    let ground = Sphere::new(Point3::vec(0.0, -100.5, -1.0), 100.0, material_ground);
    let left = Sphere::new(Point3::vec(-1.0, 0.0, -1.0), 0.5, material_left);
    let center = Sphere::new(Point3::vec(0.0, 0.0, -1.0), 0.5, material_center);
    let right = Sphere::new(Point3::vec(1.0, 0.0, -1.0), 0.5, material_right);
    let scene = &Scene::new(vec![
        Box::new(ground),
        Box::new(left),
        Box::new(center),
        Box::new(right),
    ]);

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
    let samples_per_pixel = 100;
    let max_depth = 50;

    (row0..(row0 + rows)).rev()
        .into_iter()
        .flat_map(|y| {
            (0..width).into_iter().flat_map(move |x| {
                let sampled_color =
                    (0..samples_per_pixel)
                        .into_iter()
                        .fold(Color::BLACK, move |acc, _i| {
                            let u: f64 = (x as f64 + math::rand()) / (width - 1) as f64;
                            let v: f64 = (y as f64 + math::rand()) / (height - 1) as f64;
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
