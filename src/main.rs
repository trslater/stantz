use std::env;

use image::{self, RgbImage};

fn main() {
    let args: Vec<String> = env::args().collect();

    let width: u32 = args[1].parse::<u32>().unwrap();
    let height: u32 = args[2].parse::<u32>().unwrap();

    let mut img: RgbImage = RgbImage::new(width, height);

    let mut k: u32 = 0;

    for p in img.pixels_mut() {
        let i: u32 = 255 * (k % height) / height;
        let j: u32 = 255 * (k / width) / width;

        p[0] = i as u8;
        p[1] = j as u8;
        p[2] = 255 - j as u8;

        k += 1;
    }

    let _ = img.save("img.jpg");
}
