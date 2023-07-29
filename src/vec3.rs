use crate::math_traits;
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

    pub const fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
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

    pub fn unit(&self) -> Self {
        self.clone() / self.length()
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

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x() * _rhs, self.y() * _rhs, self.z() * _rhs)
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

impl math_traits::CrossProduct for Vec3 {
    type Output = Vec3;

    fn cross(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y() * _rhs.z() - self.z() * _rhs.y(),
            self.z() * _rhs.x() - self.x() * _rhs.z(),
            self.x() * _rhs.y() - self.y() * _rhs.x(),
        )
    }
}

impl math_traits::InnerProduct for Vec3 {
    type Output = f64;

    fn dot(self, _rhs: Vec3) -> f64 {
        self.x() * _rhs.x() + self.y() * _rhs.y() + self.z() * _rhs.z()
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;
