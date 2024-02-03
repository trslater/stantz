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

const USAGE: &str =
    "cargo run --example random_spheres NUM_SPHERES NUM_LIGHTS WIDTH HEIGHT FILENAME";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("{}", USAGE);
        process::exit(1);
    }

    let num_spheres = args[1]
        .parse::<u32>()
        .expect("NUM_SPHERES must be an integer");
    let num_lights = args[2]
        .parse::<u32>()
        .expect("NUM_LIGHTS must be an integer");
    let width = args[3].parse::<u32>().expect("WIDTH must be an integer");
    let height = args[4].parse::<u32>().expect("HEIGHT must be an integer");
    let filename = &args[5];

    let mut rng = rand::thread_rng();

    let objects = (0..num_spheres)
        .map(|_| Object {
            geometry: Geometry::Sphere {
                center: Vector3::new(
                    lerp(-3.0, 3.0, rng.gen()),
                    lerp(-3.0, 3.0, rng.gen()),
                    lerp(-10.0, -6.0, rng.gen()),
                ),
                radius: lerp(0.25, 1.0, rng.gen()),
            },
            material: Material {
                diffusion: lerp(0.0, 1.0, rng.gen()),
                specularity: lerp(0.0, 1.0, rng.gen()),
                shininess: lerp(0.0, 100.0, rng.gen()) as i32,
                reflectance: lerp(0.0, 1.0, rng.gen()),
                color: Color::new(
                    lerp(0.0, 1.0, rng.gen()),
                    lerp(0.0, 1.0, rng.gen()),
                    lerp(0.0, 1.0, rng.gen()),
                ),
            },
        })
        .collect();

    let lights = (0..num_lights)
        .map(|_| Light {
            position: Vector3::new(
                lerp(-4.0, 4.0, rng.gen()),
                lerp(-4.0, 4.0, rng.gen()),
                lerp(-14.0, -2.0, rng.gen()),
            ),
            color: Color::new(
                lerp(0.0, 1.0, rng.gen()),
                lerp(0.0, 1.0, rng.gen()),
                lerp(0.0, 1.0, rng.gen()),
            ),
        })
        .collect();

    let camera = Camera {
        fov: 45.0,
        focal_length: 15.0,
    };

    render(&objects, &lights, &camera, width, height, filename);
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}
