use crate::math_traits::InnerProduct;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub trait Hittable {
    fn is_hit(&self, ray: &Ray) -> bool;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
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
}
