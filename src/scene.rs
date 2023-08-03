use crate::material;
use crate::noise;
use crate::objects;
use crate::texture;
use crate::utils;
use crate::vec3;
use crate::WorldType;
use std::sync::Arc;

pub fn simple_world() -> WorldType {
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

    let mut world: WorldType = WorldType::new();

    let dim_light_mat = Arc::new(material::DiffuseLight::with_color(vec3::Vec3::new(
        1.0, 1.0, 1.0,
    )));
    let bright_light_mat = Arc::new(material::DiffuseLight::with_color(vec3::Vec3::new(
        5.0, 5.0, 5.0,
    )));

    world.push(Arc::new(objects::XyPlane::new(
        -2.0,
        0.0,
        0.0,
        1.0,
        1.0,
        bright_light_mat,
    )));
    world.push(Arc::new(objects::XyPlane::new(
        -2.0,
        0.0,
        0.0,
        1.0,
        -2.0,
        dim_light_mat,
    )));

    world.push(Arc::new(center));
    world.push(Arc::new(ground));
    world.push(Arc::new(left));
    world.push(Arc::new(left_inner));
    world.push(Arc::new(right));
    world.push(Arc::new(up));

    world
}

pub fn random_world() -> WorldType {
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
    let mat4 = Arc::new(material::Lambertian::new(Arc::new(
        noise::PerlinNoise::new(),
    )));
    world.push(Arc::new(objects::Sphere::new(
        vec3::Point3::new(0.0, 1.0, 2.0),
        1.0,
        mat4,
    )));

    let light_mat = Arc::new(material::DiffuseLight::with_color(vec3::Vec3::new(
        1.0, 1.0, 1.0,
    )));
    world.push(Arc::new(objects::XyPlane::new(
        -4.0, 0.0, -2.0, 2.0, 2.0, light_mat,
    )));

    world
}
