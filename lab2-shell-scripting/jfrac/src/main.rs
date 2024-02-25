//! An example of generating julia fractals.
//! Adapted from https://github.com/image-rs/image

mod colormap;

use std::path::PathBuf;

use clap::Parser;
use num::Complex;

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Cli {
    /// Sets the width and height of the image.
    #[arg(short, long, value_name = "SIZE", default_value = "800")]
    size: u32,

    /// Sets the constant `c` in the equation `f(z) = z^2 + c`.
    #[arg(
        short = 'c',
        long,
        value_name = "CONSTANT",
        default_value = "-0.4 + 0.6i"
    )]
    constant: Complex<f32>,

    /// Output file path.
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let step = cli.constant;
    let size = cli.size;
    let scale = 3.0 / size as f32;

    let mut imgbuf = image::ImageBuffer::new(size, size);

    // Iterate over the coordinates and pixels of the image.
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let zx = x as f32 * scale - 1.5;
        let zy = y as f32 * scale - 1.5;
        let mut z = Complex::new(zx, zy);

        let mut i = 0u8;
        while i < 255 && z.norm() <= 2.0 {
            z = z * z + step;
            i += 1;
        }

        *pixel = image::Rgb(colormap::plasma(i));
    }

    // Save the image. The format is deduced from the path.
    imgbuf.save(cli.output).unwrap();
}
