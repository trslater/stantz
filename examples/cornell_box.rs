extern crate nalgebra as na;

use std::env;
use std::process;

use na::Vector3;

use stantz::cameras::Camera;
use stantz::geometry::Geometry;
use stantz::geometry::SphereGeometry;
use stantz::geometry::TriangleGeometry;
use stantz::lighting::Color;
use stantz::lighting::Light;
use stantz::materials::Material;
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
    let back_wall_z: f32 = -5.0;
    let front_wall_z: f32 = 1.0;

    let objects = vec![
        // Sphere 1
        (
            Geometry::Sphere(SphereGeometry {
                center: Vector3::new(-0.5, -0.5, -4.0),
                radius: 0.5,
            }),
            Material {
                diffusion: 0.3,
                specularity: 1.0,
                shininess: 50,
                reflectance: 0.75,
                color: Color::new(1.0, 1.0, 1.0),
            },
        ),
        // Sphere 2
        (
            Geometry::Sphere(SphereGeometry {
                center: Vector3::new(0.5, -0.75, -4.0),
                radius: 0.25,
            }),
            Material {
                diffusion: 0.75,
                specularity: 0.25,
                shininess: 10,
                reflectance: 0.2,
                color: Color::new(1.0, 0.0, 0.0),
            },
        ),
        // Floor
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(red_wall_x, floor_y, back_wall_z),
                b: Vector3::new(red_wall_x, floor_y, front_wall_z),
                c: Vector3::new(green_wall_x, floor_y, back_wall_z),
            }),
            Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        ),
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(green_wall_x, floor_y, front_wall_z),
                b: Vector3::new(green_wall_x, floor_y, back_wall_z),
                c: Vector3::new(red_wall_x, floor_y, front_wall_z),
            }),
            Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        ),
        // Red wall
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(red_wall_x, floor_y, front_wall_z),
                b: Vector3::new(red_wall_x, floor_y, back_wall_z),
                c: Vector3::new(red_wall_x, ceiling_y, front_wall_z),
            }),
            Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.5,
                color: Color::new(1.0, 0.0, 0.0),
            },
        ),
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
                b: Vector3::new(red_wall_x, ceiling_y, front_wall_z),
                c: Vector3::new(red_wall_x, floor_y, back_wall_z),
            }),
            Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.5,
                color: Color::new(1.0, 0.0, 0.0),
            },
        ),
        // Green wall
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(green_wall_x, floor_y, front_wall_z),
                b: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
                c: Vector3::new(green_wall_x, floor_y, back_wall_z),
            }),
            Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.5,
                color: Color::new(0.0, 1.0, 0.0),
            },
        ),
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
                b: Vector3::new(green_wall_x, floor_y, back_wall_z),
                c: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
            }),
            Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.5,
                color: Color::new(0.0, 1.0, 0.0),
            },
        ),
        // Back wall
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
                b: Vector3::new(red_wall_x, floor_y, back_wall_z),
                c: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
            }),
            Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        ),
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(green_wall_x, floor_y, back_wall_z),
                b: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
                c: Vector3::new(red_wall_x, floor_y, back_wall_z),
            }),
            Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        ),
        // Ceiling
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
                b: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
                c: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
            }),
            Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        ),
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
                b: Vector3::new(red_wall_x, ceiling_y, front_wall_z),
                c: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
            }),
            Material {
                diffusion: 1.0,
                specularity: 0.0,
                shininess: 0,
                reflectance: 0.25,
                color: Color::new(0.9, 0.8, 0.7),
            },
        ),
        // Light fixture
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(-0.5, 1.0, -3.7),
                b: Vector3::new(0.5, 1.0, -3.7),
                c: Vector3::new(-0.5, 1.0, -2.7),
            }),
            Material {
                diffusion: 0.0,
                specularity: 1.0,
                shininess: 0,
                reflectance: 0.0,
                color: Color::new(1.0, 1.0, 1.0),
            },
        ),
        (
            Geometry::Triangle(TriangleGeometry {
                a: Vector3::new(0.5, 1.0, -2.7),
                b: Vector3::new(0.5, 1.0, -3.7),
                c: Vector3::new(-0.5, 1.0, -2.7),
            }),
            Material {
                diffusion: 0.0,
                specularity: 1.0,
                shininess: 0,
                reflectance: 0.0,
                color: Color::new(1.0, 1.0, 1.0),
            },
        ),
    ];

    let lights = vec![Light {
        position: Vector3::new(0.0, 1.17, -4.0),
        color: Color::new(1.0, 1.0, 1.0),
    }];

    let camera = Camera {
        fov: 45.0,
        focal_length: 15.0,
    };

    render(&objects, &lights, &camera, width, height, filename);
}
