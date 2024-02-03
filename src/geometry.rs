use na::{Matrix3, Vector3};

use ray::Ray;

pub mod ray {
    use na::Vector3;

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
    }
}

pub enum Geometry {
    Sphere {
        center: Vector3<f32>,
        radius: f32,
    },
    Triangle {
        a: Vector3<f32>,
        b: Vector3<f32>,
        c: Vector3<f32>,
    },
}

impl Geometry {
    pub fn normal_at(&self, point: &Vector3<f32>) -> Vector3<f32> {
        match self {
            Geometry::Sphere { center, .. } => (point - center).normalize(),
            Geometry::Triangle { a, b, c } => (b - a).cross(&(c - a)).normalize(),
        }
    }

    pub fn intersection(&self, ray: &Ray) -> Option<f32> {
        match self {
            Geometry::Sphere { center, radius } => {
                // a, b, and c are standard quadratic equation coefficients

                let a: f32 = ray.direction().dot(&ray.direction());

                // Ray has ill-defined direction
                if a == 0.0 {
                    return None;
                }

                let u: Vector3<f32> = ray.origin - center;
                let b: f32 = ray.direction().dot(&u);
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

            Geometry::Triangle { a, b, c } => {
                Matrix3::from_columns(&[b - a, c - a, -ray.direction()])
                    .lu()
                    .solve(&(ray.origin - a))
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
