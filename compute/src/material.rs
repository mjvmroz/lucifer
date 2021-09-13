use derive_more::Constructor;
use crate::{color::Color, geom::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Material {
    Lambertian { albedo: Color },
    Metal {
        albedo: Color,
        fuzz: f64,
    },
}

#[derive(Debug, PartialEq, Constructor)]
pub(crate) struct ScatterRecord {
    pub(crate) scattered: Ray,
    pub(crate) attenuation: Color,
}

impl Material {
    pub(crate) fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Self::Lambertian { albedo, } => {
                let scatter_direction = Some(rec.normal + Vec3::random_in_unit_sphere())
                    .filter(|v| !v.near_zero())
                    .unwrap_or_else(|| rec.normal);
                let scattered = Ray::new(rec.p, scatter_direction);
                Some(ScatterRecord::new( scattered, *albedo ))
            },
            Self::Metal { albedo, fuzz } => {
                let reflected = r_in.direction.unit_vector().reflect(&rec.normal);
                Some(Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * fuzz))
                    .filter(|scatter| scatter.direction.dot(&rec.normal) > 0.0)
                    .map(|scattered| ScatterRecord::new(scattered, *albedo))
            },
        }
    }
}
