use std::sync::Arc;

use env_logger::Env;
use log::info;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::rtweekend::{random_double, random_double_range};
use crate::sphere::Sphere;
use crate::vec3::Vec3 as Point;
use crate::vec3::Vec3;
use crate::vec3::Vec3 as Color;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let mut world: HittableList = HittableList::default();

    let ground_material: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: Arc::clone(&ground_material),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync> = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Arc::new(Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    Arc::new(Metal { albedo, fuzz })
                } else {
                    Arc::new(Dielectric { refraction_index: 1.5 })
                };
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    mat: sphere_material,
                });
            }
        }
    }

    let material1: Arc<dyn Material + Send + Sync> =
        Arc::new(Dielectric { refraction_index: 1.5 });
    world.add(Sphere {
        center: Point::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: material1,
    });

    let material2: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Sphere {
        center: Point::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material2,
    });

    let material3: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Sphere {
        center: Point::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material3,
    });

    let mut camera: Camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.lookfrom = Point::new(13.0, 2.0, 3.0);
    camera.lookat = Point::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    camera.initialize();

    camera.render(Arc::new(world));

    info!("Done");
}
