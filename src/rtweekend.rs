use core::f64;

use rand::RngExt;

pub static PI: f64 = f64::consts::PI;
pub fn degrees_to_radiants(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::rng().random()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}
