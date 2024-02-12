use na::{Matrix3, Vector3};

use super::{aa_box::AABoxGeometry, sphere::SphereGeometry, triangle::TriangleGeometry, Geometry};

pub struct Ray {
    pub origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self {
            origin: origin,
            direction: direction.normalize(),
        }
    }

    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }

    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }

    pub fn hits_at(&self, geometry: &Geometry) -> Option<f32> {
        match geometry {
            Geometry::AABox(AABoxGeometry { min, max }) => {
                let left_t = (min.x - self.origin.x) / self.direction().x;
                let right_t = (max.x - self.origin.x) / self.direction().x;
                let bottom_t = (min.y - self.origin.y) / self.direction().y;
                let top_t = (max.y - self.origin.y) / self.direction().y;
                let back_t = (min.z - self.origin.z) / self.direction().z;
                let front_t = (max.z - self.origin.z) / self.direction().z;

                let t_min = left_t
                    .min(right_t)
                    .max(bottom_t.min(top_t))
                    .max(back_t.min(front_t));

                let t_max = left_t
                    .max(right_t)
                    .min(bottom_t.max(top_t))
                    .min(back_t.max(front_t));

                if t_max < 0.0 || t_min > t_max {
                    return None;
                }

                if t_min < 0.0 {
                    return Some(t_max);
                }

                Some(t_min)
            }
            Geometry::Sphere(SphereGeometry { center, radius }) => {
                // a, b, and c are standard quadratic equation coefficients

                let a: f32 = self.direction().dot(&self.direction());

                // Ray has ill-defined direction
                if a == 0.0 {
                    return None;
                }

                let u: Vector3<f32> = self.origin - center;
                let b: f32 = self.direction().dot(&u);
                let c: f32 = u.dot(&u) - radius.powi(2);

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
            Geometry::Triangle(TriangleGeometry { a, b, c }) => {
                Matrix3::from_columns(&[b - a, c - a, -self.direction()])
                    .lu()
                    .solve(&(self.origin - a))
                    .map(|uvt| [uvt[0], uvt[1], uvt[2]])
                    .filter(|[u, v, ..]| {
                        let w: f32 = 1.0 - u - v;

                        u >= &0.0 && u <= &1.0 && v >= &0.0 && v <= &1.0 && w >= 0.0 && w <= 1.0
                    })
                    .map(|[.., t]| t)
            }
        }
    }
}
