use derive_more::Constructor;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Copy, Clone, Constructor)]
pub(crate) struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub material: Material,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn for_ray(ray: &Ray, t: f64, p: Point3, outward_normal: Vec3, material: Material) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            t,
            p,
            material,
            normal,
            front_face,
        }
    }
}

pub(crate) trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy, PartialEq, Constructor)]
pub(crate) struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin.vec - self.center.vec;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();

        [
            (-half_b - sqrt_discriminant) / a,
            (-half_b + sqrt_discriminant) / a,
        ]
        .iter()
        .filter(|root| **root > t_min && **root < t_max)
        .map(|t| {
            let p = ray.at(*t);
            let outward_normal = (p.vec - self.center.vec) / self.radius;
            HitRecord::for_ray(ray, *t, p, outward_normal, self.material)
        })
        .next()
    }
}

#[derive(Constructor)]
pub(crate) struct Scene {
    pub objects: Vec<Sphere>,
}

impl Hittable for Scene {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects
            .iter()
            .fold((t_max, None), |(closest, res), current| {
                if let Some(hit) = current.hit(r, t_min, closest) {
                    (hit.t, Some(hit))
                } else {
                    (closest, res)
                }
            })
            .1
    }
}
