use crate::math_traits::InnerProduct;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = record.normal + Vec3::random_unit_vector();

        if scatter_dir.near_zero() {
            scatter_dir = record.normal;
        }

        let scatter_ray = Ray::new(record.hit_point, scatter_dir);
        Some((self.albedo, scatter_ray))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direction.unit().reflected(&record.normal);
        let scatter_ray = Ray::new(
            record.hit_point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        if scatter_ray.direction.dot(&record.normal) > 0.0 {
            Some((self.albedo, scatter_ray))
        } else {
            None
        }
    }
}
