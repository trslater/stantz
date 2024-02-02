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
    Plane {
        normal: Vector3<f32>,
        offset: f32,
    },
    Sphere {
        center: Vector3<f32>,
        radius: f32,
    },
    Parallelogram {
        origin: Vector3<f32>,
        a: Vector3<f32>,
        b: Vector3<f32>,
    },
}

impl Geometry {
    pub fn new_plane(normal: Vector3<f32>, offset: f32) -> Self {
        Self::Plane {
            normal: normal.normalize(),
            offset: offset,
        }
    }

    pub fn normal_at(&self, point: &Vector3<f32>) -> Vector3<f32> {
        match self {
            Geometry::Plane { normal, .. } => *normal,
            Geometry::Sphere { center, .. } => (point - center).normalize(),
            Geometry::Parallelogram { origin: _, a, b } => a.cross(b).normalize(),
        }
    }

    pub fn intersection(&self, ray: &Ray) -> Option<f32> {
        match self {
            Geometry::Plane { normal, offset } => {
                let parallelism: f32 = normal.dot(&ray.direction());

                // When ray is shining on plane orthogonally, the ray and surface normal
                // are pointing in opposite directions, so are antiparallel, i.e.,
                // parallelism is negative. If parallelism is zero, ray is parallel to
                // plane surface. If parallelism is strictly positive, ray is pointing
                // away from plane.
                if parallelism >= 0.0 {
                    return None;
                }

                Some((offset - normal.dot(&ray.origin)) / parallelism)
            }

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

            Geometry::Parallelogram { origin, a, b } => {
                Matrix3::from_columns(&[*a, *b, -ray.direction()])
                    .lu()
                    .solve(&(ray.origin - origin))
                    .map(|uvt| [uvt[0], uvt[1], uvt[2]])
                    .filter(|[u, v, ..]| u >= &0.0 && u <= &1.0 && v >= &0.0 && v <= &1.0)
                    .map(|[.., t]| t)
            }
        }
    }
}
