use std::io::{stderr, Write};

mod vec3;
use vec3::{Color, Point3};

fn main() {
    //Image
    let img_width: u64 = 256;
    let img_height: u64 = 256;

    //Render
    print!("P3\n{} {}\n255\n", img_width, img_height);

    for j in (0..img_height).rev() {
        eprint!("\rScanlines remaining: {:3}", img_height - j - 1);
        stderr().flush().unwrap();
        for i in 0..img_width {
            let pixel_color = Color::new(
                (i as f64) / ((img_width - 1) as f64),
                (j as f64) / ((img_height - 1) as f64),
                0.25);

            println!("{}", pixel_color.format_color());
        }
    }
    eprint!("Done.");
}
