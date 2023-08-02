use crate::math_traits::{CrossProduct, InnerProduct};
use crate::ray::Ray;
use crate::utils::{degrees_to_radians, random_range};
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub start_time: f64,
    pub end_time: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lens_radius = aperture / 2.0;

        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            start_time: 0.0,
            end_time: 0.0,
        }
    }

    pub fn with_timing(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        start_time: f64,
        end_time: f64,
    ) -> Self {
        let mut inst = Self::new(
            lookfrom,
            lookat,
            vup,
            fov,
            aspect_ratio,
            aperture,
            focus_dist,
        );

        inst.end_time = end_time;
        inst.start_time = start_time;

        inst
    }

    #[inline(always)]
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::with_timing(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            random_range(self.start_time, self.end_time),
        )
    }
}
