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
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut t_min_closest_so_far = Interval::new(ray_t.min, ray_t.max);

        for o in self.objects.iter() {
            if o.hit(r, &t_min_closest_so_far, &mut temp_rec) {
                hit_anything = true;
                t_min_closest_so_far.max = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
