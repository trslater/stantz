use na::Vector3;

use crate::geometry::{aa_box::AABoxGeometry, Center, NormalAt, AABB};

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
