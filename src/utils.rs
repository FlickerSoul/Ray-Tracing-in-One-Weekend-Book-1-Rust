use rand::Rng;

pub fn random() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn clamp_color(x: f64, min: f64, max: f64) -> u32 {
    (256.0 * clamp(x, min, max)) as u32
}
