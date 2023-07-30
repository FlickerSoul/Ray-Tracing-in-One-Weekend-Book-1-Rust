use crate::math_traits::InnerProduct;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub trait Hittable {
    fn is_hit(&self, ray: &Ray) -> bool;
    fn hit_ray_pos(&self, ray: &Ray) -> Option<f64>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn is_hit(&self, ray: &Ray) -> bool {
        let con: Vec3 = self.center - ray.origin;
        let touch = con.dot(ray.direction.unit()) * ray.direction.unit();
        let dis = (con - touch).length();

        if dis < 0.0 {
            return false;
        } else if dis <= self.radius {
            return true;
        } else {
            return false;
        }
    }

    fn hit_ray_pos(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let c = oc.length_squared() - self.radius * self.radius;
        let b = 2.0 * ray.direction.dot(oc);
        let a = ray.direction.length_squared();

        let inner = b * b - 4.0 * a * c;

        return if inner < 0.0 {
            None
        } else {
            Some((-b - inner.sqrt()) / (2.0 * a))
        };
    }
}
