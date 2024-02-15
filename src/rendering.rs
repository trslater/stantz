pub mod ray;

use image::RgbImage;
use na::Vector3;

use crate::{
    cameras::Camera,
    geometry::Geometry,
    lighting::{Color, Light},
    materials::Material,
    objects::Object,
};

use ray::Ray;

pub fn render(
    objects: &Vec<Object>,
    lights: &Vec<Light>,
    camera: &Camera,
    image_width: u32,
    image_height: u32,
    anti_aliasing: u32,
    filename: &String,
) {
    // Collect entries here, so it isn't repeated for every ray
    let entries: Vec<(&Geometry, &Material)> =
        objects.iter().flat_map(|object| object.iter()).collect();

    let samples_wide = image_width * anti_aliasing;
    let samples_high = image_height * anti_aliasing;
    let num_samples = samples_wide * samples_high;

    let sample_size = camera.focal_plane_height() / (samples_high as f32);
    let sample_z = -camera.focal_length;
    let origin = Vector3::new(0.0, 0.0, 0.0);

    let samples: Vec<Color> = (0..num_samples)
        .map(|k| {
            let [i, j] = [k / samples_wide, k % samples_wide];

            let sample_x = (j as f32 - (samples_wide as f32 - 1.0) / 2.0) * sample_size;
            let sample_y = -(i as f32 - (samples_high as f32 - 1.0) / 2.0) * sample_size;

            let sample_center = Vector3::new(sample_x, sample_y, sample_z);

            let ray = Ray::new(origin, sample_center);

            ray.color(&entries, lights)
        })
        .collect();

    let pixels = downsample(&samples, anti_aliasing, image_width);

    write_image(&pixels, image_width, image_height, filename);
}

fn write_image(pixels: &Vec<Color>, image_width: u32, image_height: u32, filename: &String) {
    let buffer: Vec<u8> = pixels.iter().flat_map(|p| p.as_8_bit_array()).collect();

    RgbImage::from_raw(image_width, image_height, buffer).map(|image| image.save(filename));
}

fn downsample(samples: &Vec<Color>, amount: u32, image_width: u32) -> Vec<Color> {
    let samples_per_pixel = amount.pow(2);

    let num_pixels = (samples.len() as u32) / samples_per_pixel;

    (0..num_pixels)
        .map(|k| {
            let [i, j] = [amount * (k / image_width), amount * (k % image_width)];

            (0..samples_per_pixel).fold(Color::new_black(), |color, l| {
                let [m, n] = [l / amount, l % amount];

                let sample_index = ((i + m) * amount * image_width + (j + n)) as usize;

                color + samples[sample_index] / (samples_per_pixel as f32)
            })
        })
        .collect()
}
