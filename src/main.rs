mod color;
mod math_traits;
mod vec3;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        eprintln!("Outputing {}/{}", j + 1, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let color = vec3::Color::new(
                f64::from(j) / f64::from(IMAGE_HEIGHT - 1),
                f64::from(i) / f64::from(IMAGE_WIDTH - 1),
                0.25,
            );

            color::write_color(&color, &mut std::io::stdout())
        }
    }
}
