use crate::{interval::Interval, vec3::Vec3 as Color};

pub fn write_color(pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity: Interval = Interval {
        min: 0.0,
        max: 0.999,
    };

    let rbyte = (256_f64 * intensity.clamp(r)) as i32;
    let gbyte = (256_f64 * intensity.clamp(g)) as i32;
    let bbyte = (256_f64 * intensity.clamp(b)) as i32;

    println!("{0} {1} {2}", rbyte, gbyte, bbyte);
}
