use crate::vec3::{Point3, Vec3};

use derive_more::Constructor;

#[derive(Debug, Clone, Constructor)]
pub(crate) struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub(crate) fn at(&self, t: f64) -> Point3 {
        Point3::new(self.origin.vec + self.direction * t)
    }
}
