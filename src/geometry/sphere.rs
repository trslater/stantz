use na::Vector3;

use crate::geometry::{aa_box::AABoxGeometry, ray::Ray, Center, Intersection, NormalAt, AABB};

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

impl Intersection for SphereGeometry {
    fn intersection(&self, ray: &Ray) -> Option<f32> {
        // a, b, and c are standard quadratic equation coefficients

        let a: f32 = ray.direction().dot(&ray.direction());

        // Ray has ill-defined direction
        if a == 0.0 {
            return None;
        }

        let u: Vector3<f32> = ray.origin - self.center;
        let b: f32 = ray.direction().dot(&u);
        let c: f32 = u.dot(&u) - self.radius.powi(2);

        let discriminant: f32 = b.powi(2) - c;

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
