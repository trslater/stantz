extern crate nalgebra as na;

use rand::prelude::*;
use std::env;
use std::process;

use na::Vector3;

use stantz::cameras::Camera;
use stantz::geometry::Geometry;
use stantz::lighting::Color;
use stantz::lighting::Light;
use stantz::materials::Material;
use stantz::objects::Object;
use stantz::rendering::render;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Must provide image width, height, and filename.");
        process::exit(1);
    } else if args.len() == 2 {
        println!("Must provide image height and filename.");
        process::exit(1);
    } else if args.len() == 3 {
        println!("Must provide filename.");
        process::exit(1);
    }

    let width = args[1]
        .parse::<u32>()
        .expect("Image width must be an integer");
    let height = args[2]
        .parse::<u32>()
        .expect("Image height must be an integer");
    let filename = &args[3];

    let objects = (0..10)
        .map(|_| {
            let mut rng = rand::thread_rng();

            Object {
                geometry: Geometry::Sphere {
                    center: Vector3::new(
                        lerp(-3.0, 3.0, rng.gen()),
                        lerp(-3.0, 3.0, rng.gen()),
                        lerp(0.0, 4.0, rng.gen()),
                    ),
                    radius: lerp(0.25, 1.0, rng.gen()),
                },
                material: Material {
                    diffusion: lerp(0.0, 1.0, rng.gen()),
                    specularity: lerp(0.0, 1.0, rng.gen()),
                    shininess: lerp(0.0, 100.0, rng.gen()),
                    reflectance: lerp(0.0, 1.0, rng.gen()),
                    color: Color::new(
                        lerp(0.0, 1.0, rng.gen()),
                        lerp(0.0, 1.0, rng.gen()),
                        lerp(0.0, 1.0, rng.gen()),
                    ),
                },
            }
        })
        .collect();

    let lights = vec![Light {
        position: Vector3::new(0.0, 1.17, 5.8),
        color: Color::new(1.0, 1.0, 1.0),
    }];

    let camera = Camera {
        origin: Vector3::new(0.0, 0.0, 10.0),
        fov: 45.0,
        focal_length: 15.0,
    };

    render(&objects, &lights, &camera, width, height, filename);
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}
