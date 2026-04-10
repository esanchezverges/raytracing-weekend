use std::sync::Arc;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Vec3 as Point, Vec3, dot},
};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material + Send + Sync>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = Vec3::new(0.0, 0.0, 0.0) - *outward_normal;
        }
    }

    //pub(crate) fn default() -> HitRecord {
    //Self {
    //p: Vec3::new(0.0, 0.0, 0.0),
    //normal: Vec3::new(0.0, 0.0, 0.0),
    //t: 0.0,
    //front_face: ),
    //mat: (),
    //}
    //}
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}
