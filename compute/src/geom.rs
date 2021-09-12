use derive_more::Constructor;

use crate::{ray::Ray, vec3::Point3};

pub(crate) trait Hittable {
    fn hits(&self, r: &Ray) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Constructor)]
pub(crate) struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hits(&self, r: &Ray) -> bool {
        let oc = r.origin.vec - self.center.vec;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction) * 2.0;
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}
