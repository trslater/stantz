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

    let green_wall_x: f32 = 1.5;
    let red_wall_x: f32 = -1.5;
    let ceiling_y: f32 = 1.2;
    let floor_y: f32 = -1.0;
    let back_wall_z: f32 = 5.0;
    let front_wall_z: f32 = 11.0;

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
            geometry: Geometry::Triangle {
                a: Vector3::new(red_wall_x, floor_y, back_wall_z),
                b: Vector3::new(red_wall_x, floor_y, front_wall_z),
                c: Vector3::new(green_wall_x, floor_y, back_wall_z),
            },
            material: Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        },
        Object {
            geometry: Geometry::Triangle {
                a: Vector3::new(green_wall_x, floor_y, front_wall_z),
                b: Vector3::new(green_wall_x, floor_y, back_wall_z),
                c: Vector3::new(red_wall_x, floor_y, front_wall_z),
            },
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
            geometry: Geometry::Triangle {
                a: Vector3::new(red_wall_x, floor_y, front_wall_z),
                b: Vector3::new(red_wall_x, floor_y, back_wall_z),
                c: Vector3::new(red_wall_x, ceiling_y, front_wall_z),
            },
            material: Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.5,
                color: Color::new(1.0, 0.0, 0.0),
            },
        },
        Object {
            geometry: Geometry::Triangle {
                a: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
                b: Vector3::new(red_wall_x, ceiling_y, front_wall_z),
                c: Vector3::new(red_wall_x, floor_y, back_wall_z),
            },
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
            geometry: Geometry::Triangle {
                a: Vector3::new(green_wall_x, floor_y, front_wall_z),
                b: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
                c: Vector3::new(green_wall_x, floor_y, back_wall_z),
            },
            material: Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.5,
                color: Color::new(0.0, 1.0, 0.0),
            },
        },
        Object {
            geometry: Geometry::Triangle {
                a: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
                b: Vector3::new(green_wall_x, floor_y, back_wall_z),
                c: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
            },
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
            geometry: Geometry::Triangle {
                a: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
                b: Vector3::new(red_wall_x, floor_y, back_wall_z),
                c: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
            },
            material: Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        },
        Object {
            geometry: Geometry::Triangle {
                a: Vector3::new(green_wall_x, floor_y, back_wall_z),
                b: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
                c: Vector3::new(red_wall_x, floor_y, back_wall_z),
            },
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
            geometry: Geometry::Triangle {
                a: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
                b: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
                c: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
            },
            material: Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        },
        Object {
            geometry: Geometry::Triangle {
                a: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
                b: Vector3::new(red_wall_x, ceiling_y, front_wall_z),
                c: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
            },
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
            geometry: Geometry::Triangle {
                a: Vector3::new(-0.5, 1.0, 6.3),
                b: Vector3::new(0.5, 1.0, 6.3),
                c: Vector3::new(-0.5, 1.0, 7.3),
            },
            material: Material {
                diffusion: 0.0,
                specularity: 1.0,
                shininess: 0,
                reflectance: 0.0,
                color: Color::new(1.0, 1.0, 1.0),
            },
        },
        Object {
            geometry: Geometry::Triangle {
                a: Vector3::new(0.5, 1.0, 7.3),
                b: Vector3::new(0.5, 1.0, 6.3),
                c: Vector3::new(-0.5, 1.0, 7.3),
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
        position: Vector3::new(0.0, 1.17, 6.0),
        color: Color::new(1.0, 1.0, 1.0),
    }];

    let camera = Camera {
        origin: Vector3::new(0.0, 0.0, 10.0),
        fov: 45.0,
        focal_length: 15.0,
    };

    render(&objects, &lights, &camera, width, height, filename);
}
