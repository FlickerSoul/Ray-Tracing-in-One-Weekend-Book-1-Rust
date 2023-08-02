use crate::material::Material;
use crate::math_traits::InnerProduct;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::marker::Sync;
use std::sync::Arc;

type MaterialArc = Arc<dyn Material + Sync + Send>;

pub struct HitRecord {
    pub t: f64,
    pub hit_point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: MaterialArc,
}

impl HitRecord {
    fn new(
        t: f64,
        hit_point: Point3,
        normal: Vec3,
        front_face: bool,
        material: &MaterialArc,
    ) -> Self {
        HitRecord {
            t,
            hit_point,
            normal,
            front_face,
            material: material.clone(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: MaterialArc,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: MaterialArc) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
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

        return Some(HitRecord::new(
            t,
            hit_point,
            normal,
            front_face,
            &self.material,
        ));
    }
}

impl Hittable for crate::WorldType {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        self.iter()
            .filter_map(|e| e.hit(ray, min, max))
            .min_by(|a, b| a.t.total_cmp(&b.t))
    }
}

pub struct MovingSphere {
    pub start_center: Point3,
    pub end_center: Point3,
    pub start_time: f64,
    pub end_time: f64,
    pub radius: f64,
    pub material: MaterialArc,
}

impl MovingSphere {
    pub fn new(
        start_center: Point3,
        end_center: Point3,
        start_time: f64,
        end_time: f64,
        radius: f64,
        material: MaterialArc,
    ) -> Self {
        Self {
            start_center,
            end_center,
            start_time,
            end_time,
            radius,
            material,
        }
    }

    fn moving_center(&self, time: f64) -> Point3 {
        self.start_center
            + (time - self.start_time) / (self.end_time - self.start_time)
                * (self.end_center - self.start_center)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        let hit_time = ray.time;

        let moving_center = self.moving_center(hit_time);

        let oc = ray.origin - moving_center;
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

        let out_normal = (hit_point - moving_center).unit();

        let (normal, front_face) = if out_normal.dot(&ray.direction) > 0.0 {
            (-out_normal, false)
        } else {
            (out_normal, true)
        };

        return Some(HitRecord::new(
            t,
            hit_point,
            normal,
            front_face,
            &self.material,
        ));
    }
}
