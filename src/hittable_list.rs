use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<T: Hittable + Sync + Send + 'static>(&mut self, o: T) {
        self.objects.push(Arc::new(o));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut result = None;

        for o in self.objects.iter() {
            if let Some(rec) = o.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}
