use crate::vec3::Color;
use std::io::Write;

pub fn write_color<T: Write>(color: &Color, io: &mut T) {
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
