use std::cmp::Ordering;

use image::RgbImage;
use na::Vector3;

use crate::cameras::Camera;
use crate::geometry::ray::Ray;
use crate::lighting::Color;
use crate::lighting::Light;
use crate::objects::Object;

pub fn render(
    objects: &Vec<Object>,
    lights: &Vec<Light>,
    camera: &Camera,
    image_width: u32,
    image_height: u32,
    filename: &String,
) {
    let num_pixels = image_width * image_height;

    let pixel_size = camera.focal_plane_height() / (image_height as f32);
    let pixel_z = -camera.focal_length;
    let origin = Vector3::new(0.0, 0.0, 0.0);

    let pixels: Vec<Color> = (0..num_pixels)
        .map(|k| {
            let [i, j] = [k / image_width, k % image_width];

            let pixel_y = -(i as f32 - (image_height as f32 - 1.0) / 2.0) * pixel_size;
            let pixel_x = (j as f32 - (image_width as f32 - 1.0) / 2.0) * pixel_size;

            let pixel_center = Vector3::new(pixel_x, pixel_y, pixel_z);
            let pixel_color = cast_ray(objects, lights, origin, pixel_center);

            pixel_color
        })
        .collect();

    write_image(&pixels, image_width, image_height, filename);
}

fn cast_ray(
    objects: &Vec<Object>,
    lights: &Vec<Light>,
    origin: Vector3<f32>,
    direction: Vector3<f32>,
) -> Color {
    let ray = Ray::new(origin, direction);

    objects
        .iter()
        .filter_map(|o| o.geometry.intersection(&ray).and_then(|t| Some((t, o))))
        // TODO: Does defaulting to less make sense?
        .min_by(|(ta, _), (tb, _)| ta.total_cmp(tb))
        .map(|(t, Object { geometry, material })| {
            let hit_point = ray.point_at(t);
            let hit_normal = geometry.normal_at(&hit_point);

            lights
                .iter()
                .map(|light| {
                    let light_direction = light.direction_from(&hit_point);

                    // TODO: hit_point used twice. Can we optimize?
                    let surface_diffusion = light_direction.dot(&hit_normal);
                    let surface_specularity = (light_direction - ray.direction())
                        .normalize()
                        .dot(&hit_normal);

                    light.color
                        * material.color
                        * (surface_diffusion * material.diffusion
                            + surface_specularity.powi(material.shininess) * material.specularity)
                })
                .sum()
        })
        .unwrap_or(Color::new_black())
}

fn write_image(pixels: &Vec<Color>, image_width: u32, image_height: u32, filename: &String) {
    let buffer: Vec<u8> = pixels.iter().flat_map(|p| p.as_8_bit_array()).collect();

    RgbImage::from_raw(image_width, image_height, buffer).map(|image| image.save(filename));
}
