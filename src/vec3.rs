use std::ops::{self, Index};

use rand::RngExt;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!("{0} {1} {2}", &self.e[0], &self.e[1], &self.e[2]);
        f.write_str(&str)
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn dot(o: &Vec3, u: &Vec3) -> f64 {
    o.e[0] * u.e[0] + o.e[1] * u.e[1] + o.e[2] * u.e[2]
}

#[allow(unused)]
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn print(&self) {
        println!("{0} {1} {2}", self.e[0], self.e[1], self.e[2]);
    }
    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self {
            e: [rng.random(), rng.random(), rng.random()],
        }
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::rng();
        Self {
            e: [
                rng.random_range(min..max),
                rng.random_range(min..max),
                rng.random_range(min..max),
            ],
        }
    }
    pub fn random_unit_vector() -> Self {
        loop {
            let q = Vec3::random_range(-1.0, 1.0);
            let lensq = q.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return q / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit_vector();
        if dot(&on_unit_sphere, normal) > 0.0 {
            return on_unit_sphere;
        }
        on_unit_sphere * -1.0
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self.e[0]) < s) && (f64::abs(self.e[1]) < s) && (f64::abs(self.e[2]) < s)
    }
    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - (*n * dot(v, n) * 2.0)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            e: [self.e[0] / other, self.e[1] / other, self.e[2] / other],
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}
