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
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let c = oc.length_squared() - self.radius * self.radius;
        let half_b = ray.direction.dot(&oc);
        let a = ray.direction.length_squared();

        let inner = half_b * half_b - a * c;

        if inner < 0.0 {
            return None;
        }

        let mut t = (-half_b - inner.sqrt()) / a;
        if t < min || t > max {
            t = (-half_b + inner.sqrt()) / a;
            if t < min || t > max {
                return None;
            }
        }

        let hit_point = ray.at(t);

        let out_normal = (hit_point - self.center).unit();

        let (normal, front_face) = if out_normal.dot(&ray.direction) > 0.0 {
            (-out_normal, false)
        } else {
            (out_normal, true)
        };

        return Some(HitRecord::new(t, hit_point, normal, front_face));
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        self.iter()
            .filter_map(|e| e.hit(ray, min, max))
            .min_by(|a, b| a.t.total_cmp(&b.t))
    }
}
