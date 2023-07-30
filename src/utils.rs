use rand::Rng;

pub fn random() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min, max)
}
