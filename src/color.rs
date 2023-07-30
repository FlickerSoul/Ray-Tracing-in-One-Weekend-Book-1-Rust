use crate::utils;
use crate::vec3::Color;
use std::io::Write;

pub fn write_color<T: Write>(io: &mut T, color: &Color, samples_per_pixel: u32) {
    let color = *color / samples_per_pixel as f64;

    let _ = io.write(
        format!(
            "{} {} {}\n",
            (255.99 * color.x()).floor() as u32,
            (255.99 * color.y()).floor() as u32,
            (255.99 * color.z()).floor() as u32
        )
        .as_bytes(),
    );
}
