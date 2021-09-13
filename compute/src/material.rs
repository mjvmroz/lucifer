use crate::{color::Color, geom::HitRecord, math::rand, ray::Ray, vec3::Vec3};
use derive_more::Constructor;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { ref_idx: f64 },
}

#[derive(Debug, PartialEq, Constructor)]
pub(crate) struct ScatterRecord {
    pub(crate) scattered: Ray,
    pub(crate) attenuation: Color,
}

impl Material {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    pub(crate) fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Self::Lambertian { albedo } => {
                let scatter_direction = Some(rec.normal + Vec3::random_in_unit_sphere())
                    .filter(|v| !v.near_zero())
                    .unwrap_or_else(|| rec.normal);
                let scattered = Ray::new(rec.p, scatter_direction);
                Some(ScatterRecord::new(scattered, *albedo))
            }
            Self::Metal { albedo, fuzz } => {
                let reflected = r_in.direction.unit_vector().reflect(&rec.normal);
                Some(Ray::new(
                    rec.p,
                    reflected + Vec3::random_in_unit_sphere() * fuzz,
                ))
                .filter(|scatter| scatter.direction.dot(&rec.normal) > 0.0)
                .map(|scattered| ScatterRecord::new(scattered, *albedo))
            }
            Self::Dielectric { ref_idx } => {
                let refraction_ratio = if rec.front_face {
                    1.0 / *ref_idx
                } else {
                    *ref_idx
                };

                let unit_direction = r_in.direction.unit_vector();
                let cos_theta = unit_direction.dot(&rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let scatter_direction =
                    if cannot_refract && Self::reflectance(cos_theta, refraction_ratio) > rand() {
                        unit_direction.reflect(&rec.normal)
                    } else {
                        unit_direction.refract(&rec.normal, refraction_ratio)
                    };

                Some(ScatterRecord::new(
                    Ray::new(rec.p, scatter_direction),
                    Color::WHITE,
                ))
            }
        }
    }
}
