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

use crate::camera::Camera;
use crate::color::Color;
use crate::geom::{Hittable, Scene, Sphere};
use crate::material::Material;

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

pub fn get_buffer(width: u32, height: u32, row0: u32, rows: u32) -> Vec<u8> {
    let aspect_ratio: f64 = width as f64 / height as f64;

    let scene = &generate_scene();

    let look_from = Point3::vec(13.0, 2.0, 3.0);
    let look_at = Point3::vec(0.0, 0.0, 0.0);

    // Camera
    let camera = &Camera::new(
        look_from,
        look_at,
        Vec3::y(1.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    // Image
    let samples_per_pixel = 10;
    let max_depth = 10;

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

// We generally use JS rand, but here we need stuff seeded since we're running it on every thread for the time being.
// And yes I know that Math.random isn't cryptographically random... but I'm not going to shell out for that, and
// "prng" here helps me distinguish the ideas.

fn srand(prng: &mut oorandom::Rand64) -> f64 {
    prng.rand_float()
}
fn srand_range(prng: &mut oorandom::Rand64, min: f64, max: f64) -> f64 {
    prng.rand_float() * (max - min) - min
}
fn srand_color(prng: &mut oorandom::Rand64) -> Color {
    Color {
        r: prng.rand_float(),
        g: prng.rand_float(),
        b: prng.rand_float(),
    }
}
fn srand_color_range(prng: &mut oorandom::Rand64, min: f64, max: f64) -> Color {
    Color {
        r: srand_range(prng, min, max),
        g: srand_range(prng, min, max),
        b: srand_range(prng, min, max),
    }
}

fn generate_scene() -> Scene {
    let mut prng = oorandom::Rand64::new(0);
    let random_entities: Vec<Sphere> = (-11..11)
        .into_iter()
        .flat_map(|a| {
            prng = oorandom::Rand64::new(prng.rand_u64().into());
            (-11..11).into_iter().flat_map(move |b| {
                let choose_mat = srand(&mut prng);
                let center = Point3::vec(
                    a as f64 + 0.9 * srand(&mut prng),
                    0.2,
                    b as f64 + 0.9 * srand(&mut prng),
                );

                if (center.vec - Point3::vec(4.0, 0.2, 0.0).vec).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = srand_color(&mut prng) * srand_color(&mut prng);
                        let material = Material::Lambertian { albedo };
                        Some(Sphere::new(center, 0.2, material))
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo =
                            Color::new(0.3, 0.4, 0.5) + srand_color_range(&mut prng, 0.0, 0.2);
                        let fuzz = srand_range(&mut prng, 0.0, 0.15);
                        let material = Material::Metal { albedo, fuzz };
                        Some(Sphere::new(center, 0.2, material))
                    } else {
                        // glass
                        let material = Material::Dielectric { ref_idx: 1.5 };
                        Some(Sphere::new(center, 0.2, material))
                    }
                } else {
                    None
                }
            })
        })
        .filter(|_| false)
        .collect();

    let ground = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    let glass = Material::Dielectric { ref_idx: 1.5 };
    let clay = Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    let pyrite = Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    Scene::new(
        [
            random_entities,
            vec![
                Sphere::new(Point3::vec(0.0, -1000.0, 0.0), 1000.0, ground),
                Sphere::new(Point3::vec(0.0, 1.0, 0.0), 1.0, glass),
                Sphere::new(Point3::vec(-4.0, 1.0, 0.0), 1.0, clay),
                Sphere::new(Point3::vec(4.0, 1.0, 0.0), 1.0, pyrite),
            ],
        ]
        .concat(),
    )
}
