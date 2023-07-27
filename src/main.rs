const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r = f64::from(j) / f64::from(IMAGE_HEIGHT - 1);
            let g = f64::from(i) / f64::from(IMAGE_WIDTH - 1);
            let b: f64 = 0.25;

            let ir = (255.999 * r).floor();
            let ig = (255.999 * g).floor();
            let ib = (255.999 * b).floor();

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
