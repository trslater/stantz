extern crate nalgebra as na;

use std::{env, process, time::Instant};

use na::Vector3;

use stantz::{
    cameras::Camera,
    geometry::{sphere::SphereGeometry, triangle::TriangleGeometry, Geometry},
    lighting::{Color, Light},
    materials::Material,
    objects::Object,
    rendering::render,
};

const USAGE: &str = "cargo run --example cornell_box WIDTH HEIGHT ANTI_ALIASING FILENAME";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("{}", USAGE);
        process::exit(1);
    }

    let image_width = args[1]
        .parse::<u32>()
        .expect("WIDTH must be an unsigned integer");
    let image_height = args[2]
        .parse::<u32>()
        .expect("HEIGHT must be an unsigned integer");
    let anti_aliasing = args[3]
        .parse::<u32>()
        .expect("ANTI_ALIASING must be an unsigned integer");
    let filename = &args[4];

    let green_wall_x: f32 = 1.5;
    let red_wall_x: f32 = -1.5;
    let ceiling_y: f32 = 1.2;
    let floor_y: f32 = -1.0;
    let back_wall_z: f32 = -5.0;
    let front_wall_z: f32 = 1.0;

    let sphere_1_mesh = vec![Geometry::Sphere(SphereGeometry {
        center: Vector3::new(-0.5, -0.5, -4.0),
        radius: 0.5,
    })];
    let sphere_1_material = Material {
        ambient_color: Color::new(0.0, 0.0, 0.0),
        diffuse_color: Color::new(0.3, 0.3, 0.3),
        specular_color: Color::new(1.0, 1.0, 1.0),
        shininess: 50,
        reflectance: 0.75,
    };

    let sphere_2_mesh = vec![Geometry::Sphere(SphereGeometry {
        center: Vector3::new(0.5, -0.75, -4.0),
        radius: 0.25,
    })];
    let sphere_2_material = Material {
        ambient_color: Color::new(0.0, 0.0, 0.0),
        diffuse_color: Color::new(0.75, 0.0, 0.0),
        specular_color: Color::new(0.25, 0.0, 0.0),
        shininess: 10,
        reflectance: 0.2,
    };

    let floor_mesh = vec![
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
            b: Vector3::new(red_wall_x, floor_y, back_wall_z),
            c: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
        }),
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(green_wall_x, floor_y, front_wall_z),
            b: Vector3::new(green_wall_x, floor_y, back_wall_z),
            c: Vector3::new(red_wall_x, floor_y, front_wall_z),
        }),
    ];
    let floor_material = Material {
        ambient_color: Color::new(0.0, 0.0, 0.0),
        diffuse_color: Color::new(0.9, 0.8, 0.7),
        specular_color: Color::new(0.0, 0.0, 0.0),
        shininess: 0,
        reflectance: 0.25,
    };

    let red_wall_mesh = vec![
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(red_wall_x, floor_y, front_wall_z),
            b: Vector3::new(red_wall_x, floor_y, back_wall_z),
            c: Vector3::new(red_wall_x, ceiling_y, front_wall_z),
        }),
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
            b: Vector3::new(red_wall_x, ceiling_y, front_wall_z),
            c: Vector3::new(red_wall_x, floor_y, back_wall_z),
        }),
    ];
    let red_wall_material = Material {
        ambient_color: Color::new(0.0, 0.0, 0.0),
        diffuse_color: Color::new(1.0, 0.0, 0.0),
        specular_color: Color::new(0.0, 0.0, 0.0),
        shininess: 0,
        reflectance: 0.5,
    };

    let green_wall_mesh = vec![
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(green_wall_x, floor_y, front_wall_z),
            b: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
            c: Vector3::new(green_wall_x, floor_y, back_wall_z),
        }),
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
            b: Vector3::new(green_wall_x, floor_y, back_wall_z),
            c: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
        }),
    ];
    let green_wall_material = Material {
        ambient_color: Color::new(0.0, 0.0, 0.0),
        diffuse_color: Color::new(0.0, 1.0, 0.0),
        specular_color: Color::new(0.0, 0.0, 0.0),
        shininess: 0,
        reflectance: 0.5,
    };

    let back_wall_mesh = vec![
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(red_wall_x, floor_y, back_wall_z),
            b: Vector3::new(red_wall_x, floor_y, front_wall_z),
            c: Vector3::new(green_wall_x, floor_y, back_wall_z),
        }),
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(green_wall_x, floor_y, back_wall_z),
            b: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
            c: Vector3::new(red_wall_x, floor_y, back_wall_z),
        }),
    ];

    let ceiling_mesh = vec![
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
            b: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
            c: Vector3::new(green_wall_x, ceiling_y, back_wall_z),
        }),
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(red_wall_x, ceiling_y, back_wall_z),
            b: Vector3::new(green_wall_x, ceiling_y, front_wall_z),
            c: Vector3::new(red_wall_x, ceiling_y, front_wall_z),
        }),
    ];

    let light_fixture_mesh = vec![
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(-0.5, 1.0, -3.7),
            b: Vector3::new(0.5, 1.0, -3.7),
            c: Vector3::new(-0.5, 1.0, -2.7),
        }),
        Geometry::Triangle(TriangleGeometry {
            a: Vector3::new(0.5, 1.0, -2.7),
            b: Vector3::new(0.5, 1.0, -3.7),
            c: Vector3::new(-0.5, 1.0, -2.7),
        }),
    ];
    let light_fixture_material = Material {
        ambient_color: Color::new(0.0, 0.0, 0.0),
        diffuse_color: Color::new(0.0, 0.0, 0.0),
        specular_color: Color::new(1.0, 1.0, 1.0),
        shininess: 0,
        reflectance: 0.0,
    };

    let objects: Vec<Object> = vec![
        Object {
            mesh: &sphere_1_mesh,
            material: &sphere_1_material,
        },
        Object {
            mesh: &sphere_2_mesh,
            material: &sphere_2_material,
        },
        Object {
            mesh: &floor_mesh,
            material: &floor_material,
        },
        Object {
            mesh: &red_wall_mesh,
            material: &red_wall_material,
        },
        Object {
            mesh: &green_wall_mesh,
            material: &green_wall_material,
        },
        Object {
            mesh: &back_wall_mesh,
            material: &floor_material,
        },
        Object {
            mesh: &ceiling_mesh,
            material: &floor_material,
        },
        Object {
            mesh: &light_fixture_mesh,
            material: &light_fixture_material,
        },
    ];

    let lights = vec![Light {
        position: Vector3::new(0.0, 0.8, -4.0),
        color: Color::new(1.0, 1.0, 1.0),
    }];

    let camera = Camera {
        fov: 45.0,
        focal_length: 15.0,
    };

    let now = Instant::now();
    render(
        &objects,
        &lights,
        &camera,
        image_width,
        image_height,
        anti_aliasing,
        filename,
    );
    println!("Cornell Box rendered in {:.2?}", now.elapsed());
}
