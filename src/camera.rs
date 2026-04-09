use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::{
    Vec3, Vec3 as Point, Vec3 as Color,
    color::{self, save_color, write_colors},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
};
use log::info;
use rand::RngExt;

#[derive(Default, Clone)]
pub struct Camera {
    pub center: Point,
    pub image_width: i32,
    pub pixel00_loc: Point,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub aspect_ratio: f64,
    image_height: i32,
    pub samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
}

impl Camera {
    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.max_depth = 10;
    }

    pub fn render(&self, world: Arc<HittableList>) {
        println!("P3\n{0} {1}\n255", self.image_width, self.image_height);
        let mut colors =
            vec![vec![Vec3::default(); self.image_width as usize]; self.image_height as usize];

        let mut threads: Vec<JoinHandle<(usize, Vec<Vec3>)>> = vec![];
        for j in 0..self.image_height {
            let camera = Arc::new(self.clone());
            let world = Arc::clone(&world.clone());
            threads.push(thread::spawn(move || {
                let mut result: Vec<Vec3> = vec![Vec3::default(); camera.image_width as usize];
                for i in 0..camera.image_width {
                    let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
                    let mut s: i32 = 0;
                    for _ in 0..camera.samples_per_pixel {
                        let r = camera.get_ray(i, j);
                        pixel_color += camera.ray_color(&r, &world, camera.max_depth);
                        s += 1;
                    }
                    let color_to_print = pixel_color / s as f64;
                    save_color(&mut result, i as usize, color_to_print);
                }
                info!("Finished line: {0} ", j);
                (j as usize, result)
            }));
        }

        for t in threads {
            let (j, v) = t.join().unwrap();
            colors[j] = v;
            info!("Agreggating line: {0} ", j);
        }

        write_colors(&colors);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset.x())
            + self.pixel_delta_v * (j as f64 + offset.y());
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = rand::rng();
        let x: f64 = rng.random();
        let y: f64 = rng.random();
        Vec3::new(x - 0.5, y - 0.5, 0.0)
    }

    pub fn ray_color(&self, r: &Ray, world: &HittableList, depth: i32) -> Color {
        if depth < 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec: HitRecord = HitRecord::default();
        if world.hit(
            r,
            &Interval {
                min: 0.0001,
                max: f64::INFINITY,
            },
            &mut rec,
        ) {
            let direction = rec.normal + Vec3::random_unit_vector();
            return self.ray_color(&Ray::new(rec.p, direction), world, depth - 1) * 0.7;
            //return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = r.direction();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
