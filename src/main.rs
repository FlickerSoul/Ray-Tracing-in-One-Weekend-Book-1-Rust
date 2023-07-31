mod camera;
mod color;
mod material;
mod math_traits;
mod objects;
mod ray;
mod utils;
mod vec3;

use objects::Hittable;
use std::rc::Rc;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 512;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_ITER: u32 = 5;

#[allow(dead_code)]
fn setup_world() -> Vec<Box<dyn Hittable>> {
    let ground_mat = Rc::new(material::Lambertian::new(vec3::Color::new(0.8, 0.8, 0.0)));
    let center_mat = Rc::new(material::Lambertian::new(vec3::Color::new(0.7, 0.3, 0.3)));
    let metal_shiny_mat = Rc::new(material::Metal::new(vec3::Color::new(0.8, 0.8, 0.8), 0.3));
    let metal_dull_mat = Rc::new(material::Metal::new(vec3::Color::new(0.8, 0.6, 0.2), 1.0));

    let glassy_mat = Rc::new(material::Dielectric::new(1.5));

    let ground = objects::Sphere::new(
        vec3::Vec3::new(0.0, -100.5, -1.0),
        100.0,
        ground_mat.clone(),
    );

    let center = objects::Sphere::new(vec3::Point3::new(0.0, 0.0, -1.0), 0.5, center_mat.clone());

    let left = objects::Sphere::new(vec3::Point3::new(-1.0, 0.0, -1.0), 0.5, glassy_mat.clone());
    let left_inner =
        objects::Sphere::new(vec3::Vec3::new(-1.0, 0.0, -1.0), -0.4, glassy_mat.clone());

    let right = objects::Sphere::new(
        vec3::Point3::new(1.0, 0.0, -1.0),
        0.5,
        metal_dull_mat.clone(),
    );

    let up = objects::Sphere::new(
        vec3::Point3::new(0.0, 1.0, -1.0),
        0.5,
        metal_shiny_mat.clone(),
    );

    let mut world: Vec<Box<dyn Hittable>> = vec![];

    world.push(Box::new(center));
    world.push(Box::new(ground));
    world.push(Box::new(left));
    world.push(Box::new(left_inner));
    world.push(Box::new(right));
    world.push(Box::new(up));

    world
}

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let camera = camera::Camera::default();
    let world = setup_world();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Output remaining {}", j + 1);
        for i in 0..IMAGE_WIDTH {
            let mut color = vec3::Color::zero();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + utils::random()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + utils::random()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);

                color += ray::ray_color(&ray, &world, MAX_ITER);
            }

            color::write_color(&mut std::io::stdout(), &color, SAMPLES_PER_PIXEL);
        }
    }
}
