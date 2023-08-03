use crate::vec3::{Color, Point3};
use std::sync::Arc;

pub trait Texture {
    fn color_value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub type WrappedTextureType = Arc<dyn Texture + Sync + Send>;

pub struct SolidTexture {
    pub color: Color,
}

impl SolidTexture {
    pub fn new(color: Color) -> Self {
        SolidTexture { color }
    }
}

impl Texture for SolidTexture {
    fn color_value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color.clone()
    }
}

pub struct CheckerTexture {
    pub odd: WrappedTextureType,
    pub even: WrappedTextureType,
}

impl CheckerTexture {
    pub fn new(odd: WrappedTextureType, even: WrappedTextureType) -> Self {
        CheckerTexture { odd, even }
    }

    pub fn with_color(odd: Color, even: Color) -> Self {
        CheckerTexture {
            odd: Arc::new(SolidTexture::new(odd)),
            even: Arc::new(SolidTexture::new(even)),
        }
    }
}

impl Texture for CheckerTexture {
    fn color_value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let check = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if check < 0.0 {
            self.odd.color_value(u, v, p)
        } else {
            self.even.color_value(u, v, p)
        }
    }
}
