use crate::vec3::Vec3 as Point;
use crate::vec3::Vec3;

#[derive(Default)]
pub struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Self {
        Self { orig, dir }
    }
    pub fn origin(&self) -> &Point {
        &self.orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }
    pub fn at(&self, t: f64) -> Point {
        self.orig + self.dir * t
    }
}
