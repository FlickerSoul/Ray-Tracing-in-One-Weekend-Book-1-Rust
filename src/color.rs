use crate::utils;
use crate::vec3::Color;
use std::io::Write;

pub fn write_color<T: Write>(io: &mut T, color: &Color, samples_per_pixel: u32) {
    let mut color = *color / samples_per_pixel as f64;
    color.gamma_correct(2.0);

    let _ = io.write(
        format!(
            "{} {} {}\n",
            utils::clamp_color(color.x(), 0.0, 0.999),
            utils::clamp_color(color.y(), 0.0, 0.999),
            utils::clamp_color(color.z(), 0.0, 0.999),
        )
        .as_bytes(),
    );
}
