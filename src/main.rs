mod camera;
mod color;
mod math_traits;
mod objects;
mod ray;
mod utils;
mod vec3;

use objects::Hittable;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 512;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let camera = camera::Camera::default();

    let sphere = objects::Sphere::new(vec3::Point3::new(0.0, 0.0, -1.0), 0.5);
    let ground = objects::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0);

    let mut world: Vec<Box<dyn Hittable>> = vec![];

    world.push(Box::new(sphere));
    world.push(Box::new(ground));

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Output remaining {}", j + 1);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let ray = camera.get_ray(u, v);
            let color = if let Some(record) = world.hit(&ray, 0.0, f64::INFINITY) {
                let normal = if record.front_face {
                    record.normal
                } else {
                    -record.normal
                };

                0.5 * (normal + vec3::Vec3::new(1.0, 1.0, 1.0))
            } else {
                ray::ray_color(&ray)
            };

            color::write_color(&color, &mut std::io::stdout());
        }
    }
}
