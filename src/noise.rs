use crate::utils::{random, random_uint};
use crate::vec3::{Color, Point3};

pub trait Noise {
    fn noise(&self, p: &Point3) -> f64;
}

pub struct PerlinNoise {
    pub perm_x: [usize; Self::SIZE],
    pub perm_y: [usize; Self::SIZE],
    pub perm_z: [usize; Self::SIZE],
    pub randoms: [f64; Self::SIZE],
}

impl PerlinNoise {
    const SIZE: usize = 256;

    pub fn new() -> Self {
        PerlinNoise {
            perm_x: Self::gen_perm(),
            perm_y: Self::gen_perm(),
            perm_z: Self::gen_perm(),
            randoms: Self::gen_randoms(),
        }
    }

    fn gen_randoms() -> [f64; Self::SIZE] {
        let mut randoms = [0.0; Self::SIZE];

        for i in 0..randoms.len() {
            randoms[i] = random();
        }

        randoms
    }

    pub fn gen_perm() -> [usize; Self::SIZE] {
        let mut perm: [usize; Self::SIZE] = [0; Self::SIZE];

        for i in 0..Self::SIZE {
            perm[i] = i;
        }

        Self::permute(&mut perm, Self::SIZE);

        perm
    }

    pub fn permute(arr: &mut [usize; Self::SIZE], size: usize) {
        for i in 0..size {
            let exchange = random_uint(0, size - 1);
            let tmp = arr[i];
            arr[i] = arr[exchange];
            arr[exchange] = tmp;
        }
    }
}

impl Noise for PerlinNoise {
    fn noise(&self, p: &Point3) -> f64 {
        let x = (p.x() * 4.0) as usize & (Self::SIZE - 1);
        let y = (p.y() * 4.0) as usize & (Self::SIZE - 1);
        let z = (p.z() * 4.0) as usize & (Self::SIZE - 1);

        self.randoms[self.perm_x[x] ^ self.perm_y[y] ^ self.perm_z[z]]
    }
}

impl crate::texture::Texture for PerlinNoise {
    fn color_value(&self, _u: f64, _v: f64, p: &Point3) -> crate::vec3::Color {
        Color::new(1.0, 1.0, 1.0) * self.noise(p)
    }
}
