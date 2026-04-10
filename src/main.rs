use std::sync::Arc;

use env_logger::Env;
use log::info;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Material};
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
    let mat = Lambertian {
        albedo: Color::new(0.4, 0.0, 0.0),
    };
    let mat: Arc<dyn Material + Send + Sync> = Arc::new(mat);
    world.add(Sphere {
        center: Point::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat: Arc::clone(&mat),
    });
    world.add(Sphere {
        center: Point::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: Arc::clone(&mat),
    });

    let mut camera: Camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.initialize();

    //Render
    camera.render(Arc::new(world));

    info!("Done");
}
