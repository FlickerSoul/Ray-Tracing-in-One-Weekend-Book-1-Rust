use crate::bb::{BoundingBoxHit, BoxedBoundingBoxType, AABB};
use crate::material::Material;
use crate::math_traits::{CrossProduct, InnerProduct};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::WorldType;
use std::marker::Sync;
use std::sync::Arc;

type MaterialArc = Arc<dyn Material + Sync + Send>;

pub struct HitRecord {
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub hit_point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: MaterialArc,
}

impl HitRecord {
    fn new(
        t: f64,
        u: f64,
        v: f64,
        hit_point: Point3,
        normal: Vec3,
        front_face: bool,
        material: &MaterialArc,
    ) -> Self {
        HitRecord {
            t,
            u,
            v,
            hit_point,
            normal,
            front_face,
            material: material.clone(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<BoxedBoundingBoxType>;
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

    fn get_uv(&self, p: &Vec3) -> (f64, f64) {
        let theta = -p.y().acos();
        let phi = -p.z().atan2(p.x()) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;

        (u, v)
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

        let (u, v) = self.get_uv(&normal);

        return Some(HitRecord::new(
            t,
            u,
            v,
            hit_point,
            normal,
            front_face,
            &self.material,
        ));
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<BoxedBoundingBoxType> {
        Some(Arc::new(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        )))
    }
}

impl Hittable for crate::WorldType {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        self.iter()
            .filter_map(|e| e.hit(ray, min, max))
            .min_by(|a, b| a.t.total_cmp(&b.t))
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<BoxedBoundingBoxType> {
        if self.len() == 0 {
            return None;
        }

        let mut result_box: BoxedBoundingBoxType =
            Arc::new(AABB::new(Point3::zero(), Point3::zero()));

        for item in self {
            let new_box = item.bounding_box(start_time, end_time);
            match new_box {
                None => {}
                Some(new_box) => {
                    result_box = result_box.merge(new_box);
                }
            }
        }

        Some(result_box)
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

    fn get_uv(&self, p: &Vec3) -> (f64, f64) {
        let theta = -p.y().acos();
        let phi = -p.z().atan2(p.x()) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;

        (u, v)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        let hit_time = ray.time;

        if hit_time < self.start_time || hit_time > self.end_time {
            return None;
        }

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

        let (u, v) = self.get_uv(&out_normal);

        return Some(HitRecord::new(
            t,
            u,
            v,
            hit_point,
            normal,
            front_face,
            &self.material,
        ));
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<BoxedBoundingBoxType> {
        let start_ball = AABB::new(
            self.moving_center(start_time) - Point3::new(self.radius, self.radius, self.radius),
            self.moving_center(start_time) + Point3::new(self.radius, self.radius, self.radius),
        );
        let end_ball = AABB::new(
            self.moving_center(end_time) - Point3::new(self.radius, self.radius, self.radius),
            self.moving_center(end_time) + Point3::new(self.radius, self.radius, self.radius),
        );
        return Some(start_ball.merge(Arc::new(end_ball)));
    }
}

pub struct XyPlane {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub material: MaterialArc,
}
pub struct XzPlane {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub material: MaterialArc,
}

pub struct YzPlane {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub material: MaterialArc,
}

impl XyPlane {
    const THICKNESS: f64 = 0.0001;

    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: MaterialArc) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl YzPlane {
    const THICKNESS: f64 = 0.0001;

    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: MaterialArc) -> Self {
        Self {
            z0,
            z1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl XzPlane {
    const THICKNESS: f64 = 0.0001;

    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: MaterialArc) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hittable for XyPlane {
    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<BoxedBoundingBoxType> {
        Some(Arc::new(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - Self::THICKNESS),
            Vec3::new(self.x1, self.y1, self.k + Self::THICKNESS),
        )))
    }

    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z()) / ray.direction.z();
        if t < min || t > max {
            return None;
        }

        let mut normal = Vec3::new(0.0, 0.0, 1.0);
        let front_facing = if normal.dot(&ray.direction) < 0.0 {
            true
        } else {
            normal = -normal;
            false
        };

        let hit = ray.at(t);

        if hit.x() < self.x0 || hit.x() > self.x1 || hit.y() < self.y0 || hit.y() > self.y1 {
            None
        } else {
            Some(HitRecord::new(
                t,
                (hit.x() - self.x0) / (self.x1 - self.x0),
                (hit.y() - self.y0) / (self.y1 - self.y0),
                hit,
                normal,
                front_facing,
                &self.material,
            ))
        }
    }
}
impl Hittable for YzPlane {
    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<BoxedBoundingBoxType> {
        Some(Arc::new(AABB::new(
            Vec3::new(self.k - Self::THICKNESS, self.y0, self.z0),
            Vec3::new(self.k + Self::THICKNESS, self.y1, self.z1),
        )))
    }

    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x()) / ray.direction.x();
        if t < min || t > max {
            return None;
        }

        let mut normal = Vec3::new(1.0, 0.0, 0.0);
        let front_facing = if normal.dot(&ray.direction) < 0.0 {
            true
        } else {
            normal = -normal;
            false
        };

        let hit = ray.at(t);

        if hit.y() < self.y0 || hit.y() > self.y1 || hit.z() < self.z0 || hit.z() > self.z1 {
            None
        } else {
            Some(HitRecord::new(
                t,
                (hit.y() - self.y0) / (self.y1 - self.y0),
                (hit.z() - self.z0) / (self.z1 - self.z0),
                hit,
                normal,
                front_facing,
                &self.material,
            ))
        }
    }
}

impl Hittable for XzPlane {
    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<BoxedBoundingBoxType> {
        Some(Arc::new(AABB::new(
            Vec3::new(self.x0, self.k - Self::THICKNESS, self.z0),
            Vec3::new(self.x1, self.k + Self::THICKNESS, self.z1),
        )))
    }

    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y()) / ray.direction.y();
        if t < min || t > max {
            return None;
        }

        let mut normal = Vec3::new(0.0, 1.0, 0.0);
        let front_facing = if normal.dot(&ray.direction) < 0.0 {
            true
        } else {
            normal = -normal;
            false
        };

        let hit = ray.at(t);

        if hit.x() < self.x0 || hit.x() > self.x1 || hit.z() < self.z0 || hit.z() > self.z1 {
            None
        } else {
            Some(HitRecord::new(
                t,
                (hit.x() - self.x0) / (self.x1 - self.x0),
                (hit.z() - self.z0) / (self.z1 - self.z0),
                hit,
                normal,
                front_facing,
                &self.material,
            ))
        }
    }
}

pub struct Box {
    p0: Point3,
    p1: Point3,
    pub walls: WorldType,
    pub material: MaterialArc,
}

impl Box {
    pub fn new(p0: Point3, p1: Point3, material: MaterialArc) -> Self {
        let mut walls = WorldType::new();

        // front and back
        walls.push(Arc::new(XyPlane::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            material.clone(),
        )));
        walls.push(Arc::new(XyPlane::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            material.clone(),
        )));

        // up and down
        walls.push(Arc::new(XzPlane::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            material.clone(),
        )));
        walls.push(Arc::new(XzPlane::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            material.clone(),
        )));

        // left and right
        walls.push(Arc::new(XyPlane::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            material.clone(),
        )));
        walls.push(Arc::new(XyPlane::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            material.clone(),
        )));

        Self {
            p0,
            p1,
            walls,
            material,
        }
    }
}

impl Hittable for Box {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        self.walls.hit(ray, min, max)
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<BoxedBoundingBoxType> {
        Some(Arc::new(AABB::new(self.p0, self.p1)))
    }
}
