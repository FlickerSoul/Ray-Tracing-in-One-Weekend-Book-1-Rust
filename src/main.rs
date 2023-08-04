mod bb;
mod camera;
mod color;
mod material;
mod math_traits;
mod noise;
mod objects;
mod ray;
mod scene;
mod texture;
mod utils;
mod vec3;

use objects::Hittable;
use std::sync::Arc;
use std::thread;

const ASPECT_RATIO: f64 = 1.0;
const IMAGE_WIDTH: u32 = 600;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 50;
const MAX_ITER: u32 = 120;

pub type WorldElementType = Arc<dyn Hittable + Sync + Send>;
pub type WorldType = Vec<WorldElementType>;

fn make_camera() -> camera::Camera {
    let from = vec3::Point3::new(278.0, 278.0, -800.0);
    let at = vec3::Point3::new(278.0, 278.0, 0.0);
    let up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let fov = 40.0;
    let aperture = 0.1;
    let distance_to_focus = 10.0;

    let camera = camera::Camera::with_timing(
        from,
        at,
        up,
        fov,
        ASPECT_RATIO,
        aperture,
        distance_to_focus,
        0.0,
        1.0,
    );
    camera
}

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let camera = Arc::new(make_camera());
    let world = Arc::new(scene::cornell_world());

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Output remaining {}", j + 1);
        let mut handlers = vec![];

        for i in 0..IMAGE_WIDTH {
            let world = world.clone();
            let camera = camera.clone();

            handlers.push(thread::spawn(move || {
                let mut color = vec3::Color::zero();

                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + utils::random()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + utils::random()) / (IMAGE_HEIGHT - 1) as f64;
                    let ray = camera.get_ray(u, v);

                    color += ray::ray_color(&ray, &world, MAX_ITER, &vec3::Color::zero());
                }

                color
            }));
        }

        for handler in handlers {
            let color = handler.join().unwrap();
            color::write_color(&mut std::io::stdout(), &color, SAMPLES_PER_PIXEL);
        }
    }
}
