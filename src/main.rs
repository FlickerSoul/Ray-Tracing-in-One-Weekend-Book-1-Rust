mod color;
mod math_traits;
mod objects;
mod ray;
mod vec3;

use objects::Hittable;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 512;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

const ORIGIN: vec3::Point3 = vec3::Point3::zero();
const HORIZONTAL: vec3::Point3 = vec3::Point3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const VERTICAL: vec3::Point3 = vec3::Point3::new(0.0, VIEWPORT_HEIGHT, 0.0);
const LOWER_LEFT_CORNER: vec3::Point3 = vec3::Point3::new(
    ORIGIN.x() - HORIZONTAL.x() / 2.0 - VERTICAL.x() / 2.0,
    ORIGIN.y() - HORIZONTAL.y() / 2.0 - VERTICAL.y() / 2.0,
    ORIGIN.z() - HORIZONTAL.z() / 2.0 - VERTICAL.z() / 2.0 - FOCAL_LENGTH,
);

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let sphere = objects::Sphere::new(vec3::Point3::new(0.0, 0.0, -1.0), 0.5);

    for j in 0..IMAGE_HEIGHT {
        eprintln!("Outputing {}/{}", j + 1, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let ray = ray::Ray::new(
                ORIGIN,
                LOWER_LEFT_CORNER + u * HORIZONTAL + v * VERTICAL - ORIGIN,
            );

            let color = if sphere.is_hit(&ray) {
                vec3::Color::new(1.0, 0.0, 0.0)
            } else {
                ray::ray_color(&ray)
            };

            color::write_color(&color, &mut std::io::stdout());
        }
    }
}
