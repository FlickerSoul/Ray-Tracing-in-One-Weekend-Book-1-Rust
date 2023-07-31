use crate::math_traits::{CrossProduct, InnerProduct};
use crate::utils::{random, random_range};
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub coor: [f64; 3],
}

impl Vec3 {
    pub const fn from(vec: [f64; 3]) -> Self {
        Vec3 { coor: vec }
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3::from([x, y, z])
    }

    pub fn random() -> Self {
        Vec3::new(random(), random(), random())
    }

    pub fn random_from_range(min: f64, max: f64) -> Self {
        Vec3::new(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_from_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let vec = Self::random_in_unit_sphere();

        if vec.dot(normal) > 0.0 {
            vec
        } else {
            -vec
        }
    }

    pub const fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub const fn x(&self) -> f64 {
        self.coor[0]
    }

    pub const fn y(&self) -> f64 {
        self.coor[1]
    }

    pub const fn z(&self) -> f64 {
        self.coor[2]
    }

    pub fn gamma_correct(&mut self, gamma: f64) {
        let inv_gamma = 1.0 / gamma;
        self.coor[0] = self.coor[0].powf(inv_gamma);
        self.coor[1] = self.coor[1].powf(inv_gamma);
        self.coor[2] = self.coor[2].powf(inv_gamma);
    }

    #[inline(always)]
    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        self.coor[0].abs() < EPS && self.coor[1].abs() < EPS && self.coor[2].abs() < EPS
    }

    #[inline(always)]
    pub fn reflected(&self, normal: &Vec3) -> Self {
        *self - self.dot(normal) * 2.0 * normal
    }

    #[inline(always)]
    pub fn refracted(&self, normal: &Vec3, etai_over_eta: f64) -> Self {
        let cos_theta = (-self).dot(normal).min(1.0);
        let out_perp = etai_over_eta * (self + cos_theta * normal);
        let out_para = (1.0 - out_perp.length_squared()).abs().sqrt() * -normal;

        out_perp + out_para
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x() + _rhs.x(),
            self.y() + _rhs.y(),
            self.z() + _rhs.z(),
        )
    }
}

impl<'a, 'b> ops::Add<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: &'a Vec3) -> Vec3 {
        Vec3::new(
            self.x() + _rhs.x(),
            self.y() + _rhs.y(),
            self.z() + _rhs.z(),
        )
    }
}

impl<'a> ops::Add<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x() + _rhs.x(),
            self.y() + _rhs.y(),
            self.z() + _rhs.z(),
        )
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl<'a> ops::Neg for &'a Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x() * _rhs, self.y() * _rhs, self.z() * _rhs)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x() * _rhs.x(),
            self.y() * _rhs.y(),
            self.z() * _rhs.z(),
        )
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        self + (-_rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        self * (1.0 / _rhs)
    }
}

impl<'a> ops::Mul<&'a Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: &'a Vec3) -> Vec3 {
        *_rhs * self
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        _rhs * self
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.coor[index]
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        self.coor[0] += _rhs.x();
        self.coor[1] += _rhs.y();
        self.coor[2] += _rhs.z();
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, _rhs: &Vec3) {
        self.coor[0] += _rhs.x();
        self.coor[1] += _rhs.y();
        self.coor[2] += _rhs.z();
    }
}

impl CrossProduct for Vec3 {
    type Output = Vec3;

    fn cross(&self, _rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * _rhs.z() - self.z() * _rhs.y(),
            self.z() * _rhs.x() - self.x() * _rhs.z(),
            self.x() * _rhs.y() - self.y() * _rhs.x(),
        )
    }
}

impl InnerProduct for Vec3 {
    type Output = f64;

    fn dot(&self, _rhs: &Vec3) -> f64 {
        self.x() * _rhs.x() + self.y() * _rhs.y() + self.z() * _rhs.z()
    }

    fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn unit(&self) -> Self {
        self.clone() / self.length()
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;
