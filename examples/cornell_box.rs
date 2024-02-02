extern crate nalgebra as na;

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

    let objects = vec![
        // Sphere 1
        Object {
            geometry: Geometry::Sphere {
                center: Vector3::new(-0.5, -0.5, 6.0),
                radius: 0.5,
            },
            material: Material {
                diffusion: 0.3,
                specularity: 1.0,
                shininess: 50,
                reflectance: 0.75,
                color: Color::new(1.0, 1.0, 1.0),
            },
        },
        // Sphere 2
        Object {
            geometry: Geometry::Sphere {
                center: Vector3::new(0.5, -0.75, 6.0),
                radius: 0.25,
            },
            material: Material {
                diffusion: 0.75,
                specularity: 0.25,
                shininess: 10,
                reflectance: 0.2,
                color: Color::new(1.0, 0.0, 0.0),
            },
        },
        // Floor
        Object {
            geometry: Geometry::new_plane(Vector3::new(0.0, 1.0, 0.0), -1.0),
            material: Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        },
        // Red wall
        Object {
            geometry: Geometry::new_plane(Vector3::new(1.0, 0.0, 0.0), -1.5),
            material: Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.5,
                color: Color::new(1.0, 0.0, 0.0),
            },
        },
        // Green wall
        Object {
            geometry: Geometry::new_plane(Vector3::new(-1.0, 0.0, 0.0), -1.5),
            material: Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.5,
                color: Color::new(0.0, 1.0, 0.0),
            },
        },
        // Back wall
        Object {
            geometry: Geometry::new_plane(Vector3::new(0.0, 0.0, 1.0), 5.0),
            material: Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        },
        // Front wall
        Object {
            geometry: Geometry::new_plane(Vector3::new(0.0, 0.0, -1.0), 11.0),
            material: Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        },
        // Ceiling
        Object {
            geometry: Geometry::new_plane(Vector3::new(0.0, -1.0, 0.0), -1.2),
            material: Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        },
        // Light fixture
        Object {
            geometry: Geometry::Parallelogram {
                origin: Vector3::new(-0.5, 1.0, 6.3),
                a: Vector3::new(1.0, 0.0, 0.0),
                b: Vector3::new(0.0, 0.0, 1.0),
            },
            material: Material {
                diffusion: 0.0,
                specularity: 1.0,
                shininess: 0,
                reflectance: 0.0,
                color: Color::new(1.0, 1.0, 1.0),
            },
        },
    ];

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
