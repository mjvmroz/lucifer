use crate::math::rad;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub(crate) struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub(crate) fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = Point3::ZERO;
        let horizontal = Vec3::x(viewport_width);
        let vertical = Vec3::y(viewport_height);
        let lower_left_corner =
            Point3::new(origin.vec - horizontal / 2.0 - vertical / 2.0 - Vec3::z(focal_length));

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub(crate) fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner.vec + self.horizontal * u + self.vertical * v - self.origin.vec,
        )
    }
}
