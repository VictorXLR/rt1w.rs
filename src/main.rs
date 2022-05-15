mod color;
mod vec3;

use crate::color::write_color;
use crate::vec3::Color;
use std::io::{self, Write};

fn main() {
    // Image
    const IMAGE_WIDTH: i64 = 256;
    const IMAGE_HEIGHT: i64 = 256;

    // Render
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let r: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let pixel_color = Color::new(r, g, b);
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone.")
}
