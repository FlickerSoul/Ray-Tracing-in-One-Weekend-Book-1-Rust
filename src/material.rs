use crate::math_traits::InnerProduct;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidTexture, WrappedTextureType};
use crate::utils::random;
use crate::vec3::{Color, Vec3};
use std::sync::Arc;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: WrappedTextureType,
}

impl Lambertian {
    pub fn new(albedo: WrappedTextureType) -> Self {
        Lambertian { albedo }
    }

    pub fn with_color(albedo: Color) -> Self {
        Lambertian {
            albedo: Arc::new(SolidTexture::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = record.normal + Vec3::random_unit_vector();

        if scatter_dir.near_zero() {
            scatter_dir = record.normal;
        }

        let scatter_ray = Ray::with_timing(record.hit_point, scatter_dir, ray.time);
        Some((
            self.albedo
                .color_value(record.u, record.v, &record.hit_point),
            scatter_ray,
        ))
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
        let scatter_ray = Ray::with_timing(
            record.hit_point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            ray.time,
        );

        if scatter_ray.direction.dot(&record.normal) > 0.0 {
            Some((self.albedo, scatter_ray))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        const attenuation: Color = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let ray_unit_dir = ray.direction.unit();
        let cos_theta = (-ray_unit_dir).dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let refracted =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                ray_unit_dir.reflected(&record.normal)
            } else {
                ray_unit_dir.refracted(&record.normal, refraction_ratio)
            };

        return Some((
            attenuation,
            Ray::with_timing(record.hit_point, refracted, ray.time),
        ));
    }
}
