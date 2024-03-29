pub mod aa_box;
pub mod sphere;
pub mod triangle;

use na::Vector3;

use crate::rendering::ray::Ray;

use aa_box::AABoxGeometry;
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

pub trait Intersection<T, U> {
    type Argument;
    type Output;

    fn intersection(&self, other: &Self::Argument) -> Option<Self::Output>;
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

impl Intersection<Ray, f32> for Geometry {
    type Argument = Ray;
    type Output = f32;

    fn intersection(&self, other: &Self::Argument) -> Option<Self::Output> {
        match self {
            Geometry::AABox(aa_box) => aa_box.intersection(other),
            Geometry::Sphere(sphere) => sphere.intersection(other),
            Geometry::Triangle(triangle) => triangle.intersection(other),
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
