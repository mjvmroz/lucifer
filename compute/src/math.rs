use std::f64::consts::PI;

use oorandom::Rand64;

pub(crate) fn rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub(crate) fn rand_range(rng: &mut Rand64, min: f64, max: f64) -> f64 {
    rng.rand_float() * (max - min) + min
}
