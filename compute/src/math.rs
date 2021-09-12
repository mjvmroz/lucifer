use std::f64::consts::PI;

pub(crate) fn rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub(crate) fn js_rand() -> f64 {
    unsafe { js_sys::Math::random() }
}

pub(crate) fn js_rand_range(min: f64, max: f64) -> f64 {
    js_rand() * (max - min) + min
}
