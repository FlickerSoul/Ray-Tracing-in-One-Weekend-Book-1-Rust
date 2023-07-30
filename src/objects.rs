use crate::math_traits::InnerProduct;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub t: f64,
    pub hit_point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    fn new(t: f64, hit_point: Point3, normal: Vec3, front_face: bool) -> Self {
        HitRecord {
            t,
            hit_point,
            normal,
            front_face,
        }
    }
}

pub trait Hittable {
    fn is_hit(&self, ray: &Ray) -> bool;
    fn hit_ray_pos(&self, ray: &Ray) -> Option<f64>;
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord>;
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
        let touch = con.dot(&ray.direction.unit()) * ray.direction.unit();
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
        let b = 2.0 * ray.direction.dot(&oc);
        let a = ray.direction.length_squared();

        let inner = b * b - 4.0 * a * c;

        return if inner < 0.0 {
            None
        } else {
            Some((-b - inner.sqrt()) / (2.0 * a))
        };
    }

    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let c = oc.length_squared() - self.radius * self.radius;
        let b = 2.0 * ray.direction.dot(&oc);
        let a = ray.direction.length_squared();

        let inner = b * b - 4.0 * a * c;

        if inner < 0.0 {
            return None;
        }

        let mut t = (-b - inner.sqrt()) / (2.0 * a);
        if t < min || t > max {
            t = (-b + inner.sqrt()) / (2.0 * a);
            if t < min || t > max {
                return None;
            }
        }

        let hit_point = ray.at(t);

        let out_normal = (hit_point - self.center).unit();

        let (normal, front_face) = if out_normal.dot(&ray.direction) < 0.0 {
            (-out_normal, false)
        } else {
            (out_normal, true)
        };

        return Some(HitRecord::new(t, hit_point, normal, front_face));
    }
}
