use crate::math::rad;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub(crate) struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub(crate) fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64, // We're not using real lenses? Disgusting.
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from.vec - look_at.vec).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner =
            Point3::new(origin.vec - horizontal / 2.0 - vertical / 2.0 - w * focus_dist);

        let lens_radius = aperture / 2.0;

        Self {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub(crate) fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disc() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            Point3::new(self.origin.vec + offset),
            self.lower_left_corner.vec + self.horizontal * s + self.vertical * t
                - self.origin.vec
                - offset,
        )
    }
}
