use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::{
    Vec3, Vec3 as Point, Vec3 as Color,
    color::{self, save_color, write_colors},
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    rtweekend::degrees_to_radiants,
    vec3::{self, cross, unit_vector},
};
use log::info;
use rand::RngExt;

#[derive(Default, Clone)]
pub struct Camera {
    pub max_depth: i32,
    image_height: i32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pixel_samples_scale: f64,
    pub aspect_ratio: f64,
    pub vfov: f64,
    pub center: Point,
    pub pixel00_loc: Point,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.center = self.lookfrom;

        let theta = degrees_to_radiants(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = unit_vector(&(self.lookfrom - self.lookat));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = self.v * -1.0 * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.w * self.focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;

        let defocus_radius =
            f64::tan(degrees_to_radiants(self.defocus_angle / 2.0)) * self.focus_dist;
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
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
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = vec3::random_unit_in_disk();
        self.center + (self.defocus_disk_u * p[0]) + (self.defocus_disk_v * p[1])
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
        if let Some(rec) = world.hit(
            r,
            &Interval {
                min: 0.0001,
                max: f64::INFINITY,
            },
        ) {
            let mut scattered: Ray = Ray::default();
            let mut attenuation: Vec3 = Vec3::default();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, world, depth - 1);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
