use na::Vector3;

use crate::rendering::ray::Ray;

use super::{aa_box::AABoxGeometry, Center, Intersection, NormalAt, AABB};

#[derive(Debug, Clone)]
pub struct SphereGeometry {
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl Center for SphereGeometry {
    fn center(&self) -> Vector3<f32> {
        self.center
    }
}

impl NormalAt for SphereGeometry {
    fn normal_at(&self, point: &Vector3<f32>) -> Vector3<f32> {
        (point - self.center).normalize()
    }
}

impl Intersection<Ray, f32> for SphereGeometry {
    type Argument = Ray;
    type Output = f32;

    fn intersection(&self, other: &Self::Argument) -> Option<Self::Output> {
        // a, b, and c are standard quadratic equation coefficients

        let a = other.direction().dot(&other.direction());

        // Ray has ill-defined direction
        if a == 0.0 {
            return None;
        }

        let u: Vector3<Self::Output> = other.origin - self.center;
        let b = other.direction().dot(&u);
        let c = u.dot(&u) - self.radius.powi(2);

        let discriminant = b.powi(2) - c;

        // Misses
        if discriminant < 0.0 {
            return None;
        }

        // Just grazes surface (i.e., tangent to surface)
        if discriminant == 0.0 {
            return Some(-b);
        }

        // Since second_term is always positive, (-) root is always smaller,
        // i.e., closer to camera
        Some(-b - discriminant.sqrt())
    }
}

impl AABB for SphereGeometry {
    fn aabb(&self) -> AABoxGeometry {
        AABoxGeometry {
            min: Vector3::new(
                self.center.x - self.radius,
                self.center.y - self.radius,
                self.center.z - self.radius,
            ),
            max: Vector3::new(
                self.center.x + self.radius,
                self.center.y + self.radius,
                self.center.z + self.radius,
            ),
        }
    }
}
