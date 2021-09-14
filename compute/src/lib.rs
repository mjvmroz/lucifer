mod camera;
mod color;
mod geom;
mod material;
mod math;
mod ray;
mod utils;
mod vec3;

use std::f64::consts::PI;

use ray::Ray;
use vec3::{Point3, Vec3};
use wasm_bindgen::prelude::*;

use crate::camera::Camera;
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
                record.material.scatter(ray, &record).map_or_else(
                    || Color::BLACK,
                    |scatter_record| {
                        scatter_record.attenuation
                            * ray_color(&scatter_record.scattered, scene, depth - 1)
                    },
                )
            },
        )
    }
}

#[wasm_bindgen]
pub fn get_buffer(width: u32, height: u32, row0: u32, rows: u32) -> Vec<u8> {
    let aspect_ratio: f64 = width as f64 / height as f64;

    // Scene
    let ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let glass = Material::Dielectric { ref_idx: 1.5 };
    let matte_blue = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let polished_brass = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    let matte_red = Material::Lambertian { albedo: Color::RED };

    let R = (PI / 4.0).cos();
    let scene = &Scene::new(vec![
        Box::new(Sphere::new(Point3::vec(0.0, -100.5, -1.0), 100.0, ground)),
        Box::new(Sphere::new(Point3::vec(-1.0, 0.0, -1.0), 0.5, glass)),
        Box::new(Sphere::new(Point3::vec(-1.0, 0.0, -1.0), -0.4, glass)),
        Box::new(Sphere::new(Point3::vec(0.0, 0.0, -1.0), 0.5, matte_blue)),
        Box::new(Sphere::new(
            Point3::vec(1.0, 0.0, -1.0),
            0.5,
            polished_brass,
        )),
        // Box::new(Sphere::new(Point3::vec(-R, 0.0, -1.0), R, matte_blue)),
        // Box::new(Sphere::new(Point3::vec(R, 0.0, -1.0), R, matte_red)),
    ]);

    // Camera
    let camera = &Camera::new(
        Point3::vec(2.0, 2.0, -4.0),
        Point3::vec(0.0, 0.0, -1.0),
        Vec3::y(1.0),
        20.0,
        aspect_ratio,
    );

    // Image
    let samples_per_pixel = 10;
    let max_depth = 15;

    (row0..(row0 + rows))
        .rev()
        .into_iter()
        .flat_map(|y| {
            (0..width).into_iter().flat_map(move |x| {
                let sampled_color =
                    (0..samples_per_pixel)
                        .into_iter()
                        .fold(Color::BLACK, move |acc, _i| {
                            let u: f64 = (x as f64 + math::rand()) / (width - 1) as f64;
                            let v: f64 = (y as f64 + math::rand()) / (height - 1) as f64;
                            let r = camera.get_ray(u, v);
                            acc + ray_color(&r, scene, max_depth)
                        });
                (sampled_color / samples_per_pixel as f64).sqrt().to_bytes()
            })
        })
        .collect()
}
