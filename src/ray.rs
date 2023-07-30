use crate::math_traits::InnerProduct;
use crate::objects::Hittable;
use crate::vec3;

pub struct Ray {
    pub origin: vec3::Point3,
    pub direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: vec3::Point3, direction: vec3::Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        self.origin + t * self.direction
    }
}

pub fn background(ray: &Ray) -> vec3::Color {
    let unit_direction = ray.direction.unit();

    let t = 0.5 * (unit_direction.y() + 1.0); // going to work with focal length

    (1.0 - t) * vec3::Color::new(1.0, 1.0, 1.0) + t * vec3::Color::new(0.5, 0.7, 1.0)
}

pub fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> vec3::Color {
    if let Some(record) = world.hit(&ray, 0.0, f64::INFINITY) {
        let normal = if record.front_face {
            record.normal
        } else {
            -record.normal
        };

        0.5 * (normal + vec3::Vec3::new(1.0, 1.0, 1.0))
    } else {
        background(&ray)
    }
}
