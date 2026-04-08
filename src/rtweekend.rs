use core::f64;

pub static PI: f64 = f64::consts::PI;
pub fn degrees_to_radiants(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
