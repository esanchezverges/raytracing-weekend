use crate::{interval::Interval, vec3::Vec3, vec3::Vec3 as Color};

fn linear_to_gamma(color: f64) -> f64 {
    if color > 0.0 {
        return color.sqrt();
    }
    color
}

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

pub fn write_colors(v: &Vec<Vec<Vec3>>) {
    use std::io::{self, Write};
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for j in v.iter() {
        for i in j.iter() {
            writeln!(out, "{} {} {}", i.x() as i32, i.y() as i32, i.z() as i32).unwrap();
        }
    }
}

pub fn save_color(v: &mut Vec<Vec3>, i: usize, pixel_color: Color) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity: Interval = Interval {
        min: 0.0,
        max: 0.999,
    };

    let rbyte = (256_f64 * intensity.clamp(r)) as i32;
    let gbyte = (256_f64 * intensity.clamp(g)) as i32;
    let bbyte = (256_f64 * intensity.clamp(b)) as i32;

    v[i] = Vec3::new(rbyte as f64, gbyte as f64, bbyte as f64);
}
