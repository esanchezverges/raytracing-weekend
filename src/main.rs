use std::sync::Arc;

use env_logger::Env;
use log::info;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Vec3 as Point;
use crate::vec3::Vec3;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
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
    world.add(Sphere {
        center: Point::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Point::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });

    let mut camera: Camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 1000;
    camera.initialize();

    //Render
    camera.render(Arc::new(world));

    info!("Done");
}
