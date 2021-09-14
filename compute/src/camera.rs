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
    pub(crate) fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from.vec - look_at.vec).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = Point3::new(origin.vec - horizontal / 2.0 - vertical / 2.0 - w);

        Self {
            origin: look_from,
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
