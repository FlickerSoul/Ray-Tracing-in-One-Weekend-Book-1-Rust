use crate::math_traits::InnerProduct;
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

pub fn ray_color(ray: &Ray) -> vec3::Color {
    let unit_direction = ray.direction.unit();

    let t = 0.5 * (unit_direction.y() + 1.0); // going to work with focal length

    (1.0 - t) * vec3::Color::new(1.0, 1.0, 1.0) + t * vec3::Color::new(0.5, 0.7, 1.0)
}
