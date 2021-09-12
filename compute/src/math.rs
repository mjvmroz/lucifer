use std::f64::consts::PI;

pub(crate) fn rad(deg: f64) -> f64 {
    deg * PI / 180.0
}
