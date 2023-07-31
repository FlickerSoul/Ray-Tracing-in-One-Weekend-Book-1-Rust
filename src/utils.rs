use rand::Rng;

#[inline(always)]
pub fn random() -> f64 {
    rand::thread_rng().gen()
}

#[inline(always)]
pub fn random_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

#[inline(always)]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[inline(always)]
pub fn clamp_color(x: f64, min: f64, max: f64) -> u32 {
    (256.0 * clamp(x, min, max)) as u32
}

#[inline(always)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
