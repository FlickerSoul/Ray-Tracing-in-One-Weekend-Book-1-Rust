use crate::objects::Hittable;
use crate::ray::Ray;
use crate::utils::random_uint;
use crate::vec3::Point3;
use crate::{WorldElementType, WorldType};
use std::cmp;
use std::sync::Arc;

pub type BoxedBoundingBoxType = Arc<dyn BoundingBoxHit + Sync + Send>;

pub trait BoundingBoxHit {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> bool;

    fn min(&self) -> &Point3;
    fn max(&self) -> &Point3;

    fn merge(&self, bounding_box: BoxedBoundingBoxType) -> BoxedBoundingBoxType;
}

pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }
}

impl BoundingBoxHit for AABB {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> bool {
        let mut t_min = min;
        let mut t_max = max;

        for i in 0..3 {
            let min = (self.min[i] - ray.origin[i]) / ray.direction[i];
            let max = (self.max[i] - ray.origin[i]) / ray.direction[i];

            let t0 = min.min(max);
            let t1 = min.max(max);

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    fn min(&self) -> &Point3 {
        &self.min
    }

    fn max(&self) -> &Point3 {
        &self.max
    }

    fn merge(&self, bounding_box: BoxedBoundingBoxType) -> BoxedBoundingBoxType {
        Arc::new(Self::new(
            self.min().merge_min(bounding_box.min()),
            self.max().merge_max(bounding_box.max()),
        ))
    }
}

struct BvhNode {
    pub left: WorldElementType,
    pub right: WorldElementType,
    pub bounding_box: BoxedBoundingBoxType,
}

fn box_comparator(
    axis: usize,
) -> Box<dyn Fn(&WorldElementType, &WorldElementType) -> cmp::Ordering> {
    assert!(axis < 3);

    Box::new(move |a: &WorldElementType, b: &WorldElementType| {
        let a_box = a.bounding_box(0.0, 0.0);
        let b_box = b.bounding_box(0.0, 0.0);

        assert!(a_box.is_some() || b_box.is_some());

        a_box.unwrap().min()[axis]
            .partial_cmp(&b_box.unwrap().min()[axis])
            .unwrap()
    })
}

impl BvhNode {
    pub fn new(
        objects: &'static WorldType,
        start: usize,
        end: usize,
        start_time: f64,
        end_time: f64,
    ) -> Self {
        let left: WorldElementType;
        let right: WorldElementType;

        let object_span = end - start;
        assert!(object_span > 0);

        let axis = random_uint(0, 2);

        let comparator = box_comparator(axis);

        match object_span {
            1 => {
                right = objects[start].clone();
                left = objects[start].clone();
            }
            2 => match comparator(&objects[start], &objects[end]) {
                cmp::Ordering::Less => {
                    left = objects[start].clone();
                    right = objects[end].clone();
                }
                _ => {
                    right = objects[start].clone();
                    left = objects[end].clone();
                }
            },
            _ => {
                let mid = (start + end) / 2;
                left =
                    Arc::new(Self::new(objects, start, start + mid, start_time, end_time)).clone();
                right = Arc::new(Self::new(objects, mid, end, start_time, end_time)).clone()
            }
        }

        let bounding_box = left
            .bounding_box(start_time, end_time)
            .unwrap()
            .merge(right.bounding_box(start_time, end_time).unwrap());

        BvhNode {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<crate::objects::HitRecord> {
        if !self.bounding_box.hit(ray, min, max) {
            return None;
        }

        let hit_left = self.left.hit(ray, min, max);
        let hit_right = if hit_left.is_none() {
            self.right.hit(ray, min, max)
        } else {
            self.right.hit(ray, min, ray.time)
        };

        return if hit_left.is_some() {
            hit_left
        } else {
            hit_right
        };
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<BoxedBoundingBoxType> {
        Some(self.bounding_box.clone())
    }
}
