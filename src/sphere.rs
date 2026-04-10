use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::{
    ray::Ray,
    vec3::{Vec3 as Point, dot},
};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub mat: Arc<dyn Material + Send + Sync>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let mut rec = HitRecord {
            t,
            p,
            mat: Arc::clone(&self.mat),
            normal: outward_normal,
            front_face: false,
        };
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }
}
