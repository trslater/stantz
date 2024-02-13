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
    filename: &String,
) {
    // Collect entries here, so it isn't repeated for every ray
    let entries: Vec<(&Geometry, &Material)> =
        objects.iter().flat_map(|object| object.iter()).collect();

    let num_pixels = image_width * image_height;

    let pixel_size = camera.focal_plane_height() / (image_height as f32);
    let pixel_z = -camera.focal_length;
    let origin = Vector3::new(0.0, 0.0, 0.0);

    let pixels: Vec<Color> = (0..num_pixels)
        .map(|k| {
            let [i, j] = [k / image_width, k % image_width];

            let pixel_x = (j as f32 - (image_width as f32 - 1.0) / 2.0) * pixel_size;
            let pixel_y = -(i as f32 - (image_height as f32 - 1.0) / 2.0) * pixel_size;

            let pixel_center = Vector3::new(pixel_x, pixel_y, pixel_z);

            let ray = Ray::new(origin, pixel_center);

            ray.color(&entries, lights)
        })
        .collect();

    write_image(&pixels, image_width, image_height, filename);
}

fn write_image(pixels: &Vec<Color>, image_width: u32, image_height: u32, filename: &String) {
    let buffer: Vec<u8> = pixels.iter().flat_map(|p| p.as_8_bit_array()).collect();

    RgbImage::from_raw(image_width, image_height, buffer).map(|image| image.save(filename));
}
