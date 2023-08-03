mod bb;
mod camera;
mod color;
mod material;
mod math_traits;
mod objects;
mod ray;
mod texture;
mod utils;
mod vec3;

use objects::Hittable;
use std::sync::Arc;
use std::thread;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: u32 = 500;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 10;
const MAX_ITER: u32 = 5;

pub type WorldElementType = Arc<dyn Hittable + Sync + Send>;
pub type WorldType = Vec<WorldElementType>;

#[allow(dead_code)]
fn setup_simple_world() -> WorldType {
    let ground_mat = Arc::new(material::Lambertian::with_color(vec3::Color::new(
        0.8, 0.8, 0.0,
    )));
    let center_mat = Arc::new(material::Lambertian::with_color(vec3::Color::new(
        0.7, 0.3, 0.3,
    )));
    let metal_shiny_mat = Arc::new(material::Metal::new(vec3::Color::new(0.8, 0.8, 0.8), 0.3));
    let metal_dull_mat = Arc::new(material::Metal::new(vec3::Color::new(0.8, 0.6, 0.2), 1.0));

    let glassy_mat = Arc::new(material::Dielectric::new(1.5));

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

    let mut world: WorldType = vec![];

    world.push(Arc::new(center));
    world.push(Arc::new(ground));
    world.push(Arc::new(left));
    world.push(Arc::new(left_inner));
    world.push(Arc::new(right));
    world.push(Arc::new(up));

    world
}

fn setup_world(seed: Option<usize>) -> WorldType {
    if let Some(seed) = seed {}

    let mut world = WorldType::new();

    let ground_tx = Arc::new(texture::CheckerTexture::with_color(
        vec3::Color::new(0.2, 0.3, 0.1),
        vec3::Color::new(0.9, 0.9, 0.9),
    ));

    let ground_mat = Arc::new(material::Lambertian::new(ground_tx));
    world.push(Arc::new(objects::Sphere::new(
        vec3::Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));

    const BOUND: isize = 11;

    for a in -BOUND..BOUND {
        for b in -BOUND..BOUND {
            let mat_choice = utils::random();

            let center = vec3::Point3::new(
                a as f64 + 0.9 * utils::random(),
                0.2,
                b as f64 + 0.9 * utils::random(),
            );

            if mat_choice < 0.8 {
                let albedo = vec3::Color::random() * vec3::Color::random();
                let sphere_mat = Arc::new(material::Lambertian::with_color(albedo));
                let center2 = center + vec3::Vec3::new(0.0, utils::random_range(0.0, 0.5), 0.0);
                world.push(Arc::new(objects::MovingSphere::new(
                    center, center2, 0.0, 1.0, 0.2, sphere_mat,
                )));
            } else if mat_choice < 0.95 {
                let albedo = vec3::Color::random_from_range(0.5, 1.0);
                let fuzz = utils::random_range(0.0, 0.5);
                let sphere_mat = Arc::new(material::Metal::new(albedo, fuzz));
                world.push(Arc::new(objects::Sphere::new(center, 0.2, sphere_mat)));
            } else {
                let sphere_mat = Arc::new(material::Dielectric::new(1.5));
                world.push(Arc::new(objects::Sphere::new(center, 0.2, sphere_mat)));
            }
        }
    }

    let mat1 = Arc::new(material::Dielectric::new(1.5));
    world.push(Arc::new(objects::Sphere::new(
        vec3::Point3::new(0.0, 1.0, 0.0),
        1.0,
        mat1,
    )));
    let mat2 = Arc::new(material::Lambertian::with_color(vec3::Color::new(
        0.4, 0.2, 0.1,
    )));
    world.push(Arc::new(objects::Sphere::new(
        vec3::Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    )));
    let mat3 = Arc::new(material::Metal::new(vec3::Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Arc::new(objects::Sphere::new(
        vec3::Point3::new(4.0, 1.0, 0.0),
        1.0,
        mat3,
    )));

    world
}

fn make_camera() -> camera::Camera {
    let from = vec3::Point3::new(13.0, 2.0, 3.0);
    let at = vec3::Point3::new(0.0, 0.0, 0.0);
    let up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let fov = 20.0;
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
    let world = Arc::new(setup_world(None));

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

                    color += ray::ray_color(&ray, &world, MAX_ITER);
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
