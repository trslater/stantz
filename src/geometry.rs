pub mod aa_box;
pub mod ray;
pub mod sphere;
pub mod triangle;

use na::Vector3;

use aa_box::AABoxGeometry;
use ray::Ray;
use sphere::SphereGeometry;
use triangle::TriangleGeometry;

#[derive(Debug, Clone)]
pub enum Geometry {
    AABox(AABoxGeometry),
    Sphere(SphereGeometry),
    Triangle(TriangleGeometry),
}

pub trait Center {
    fn center(&self) -> Vector3<f32>;
}

pub trait NormalAt {
    fn normal_at(&self, point: &Vector3<f32>) -> Vector3<f32>;
}

pub trait Intersection {
    fn intersection(&self, ray: &Ray) -> Option<f32>;
}

pub trait AABB {
    fn aabb(&self) -> AABoxGeometry;
}

impl Center for Geometry {
    fn center(&self) -> Vector3<f32> {
        match self {
            Geometry::AABox(aa_box) => aa_box.center(),
            Geometry::Sphere(sphere) => sphere.center(),
            Geometry::Triangle(triangle) => triangle.center(),
        }
    }
}

impl NormalAt for Geometry {
    fn normal_at(&self, point: &Vector3<f32>) -> Vector3<f32> {
        match self {
            Geometry::AABox(aa_box) => aa_box.normal_at(point),
            Geometry::Sphere(sphere) => sphere.normal_at(point),
            Geometry::Triangle(triangle) => triangle.normal_at(point),
        }
    }
}

impl Intersection for Geometry {
    fn intersection(&self, ray: &Ray) -> Option<f32> {
        match self {
            Geometry::AABox(aa_box) => aa_box.intersection(ray),
            Geometry::Sphere(sphere) => sphere.intersection(ray),
            Geometry::Triangle(triangle) => triangle.intersection(ray),
        }
    }
}
