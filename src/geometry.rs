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

#[derive(Debug)]
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
    AABox {
        min: Vector3<f32>,
        max: Vector3<f32>,
    },
}

impl Geometry {
    pub fn center(&self) -> Vector3<f32> {
        match self {
            Geometry::Sphere { center, .. } => *center,
            Geometry::Triangle { a, b, c } => Vector3::new(
                [a.x, b.x, c.x].iter().sum::<f32>() / 3.0,
                [a.y, b.y, c.y].iter().sum::<f32>() / 3.0,
                [a.z, b.z, c.z].iter().sum::<f32>() / 3.0,
            ),
            Geometry::AABox { min, max } => {
                Vector3::new(max.x - min.x, max.y - min.y, max.z - min.z)
            }
        }
    }

    pub fn normal_at(&self, point: &Vector3<f32>) -> Vector3<f32> {
        match self {
            Geometry::Sphere { center, .. } => (point - center).normalize(),
            Geometry::Triangle { a, b, c } => (b - a).cross(&(c - a)).normalize(),
            Geometry::AABox { min, max } => {
                // TODO: Implement edge/corner cases (literally)
                let is_left_of = point.x <= min.x;
                let is_right_of = point.x >= max.x;
                let is_below = point.y <= min.y;
                let is_above = point.y >= max.y;
                let is_behind = point.z <= min.z;
                let is_in_front_of = point.z >= max.z;

                let is_in_x_bounds = point.x >= min.x && point.x <= max.x;
                let is_in_y_bounds = point.y >= min.y && point.y <= max.y;
                let is_in_z_bounds = point.z >= min.z && point.z <= max.z;

                if is_in_x_bounds && is_in_y_bounds {
                    if is_behind {
                        return Vector3::new(0.0, 0.0, -1.0);
                    }
                    if is_in_front_of {
                        return Vector3::new(0.0, 0.0, 1.0);
                    }
                }
                if is_in_x_bounds && is_in_z_bounds {
                    if is_below {
                        return Vector3::new(0.0, -1.0, 0.0);
                    }
                    if is_above {
                        return Vector3::new(0.0, 1.0, 0.0);
                    }
                }
                if is_in_y_bounds && is_in_z_bounds {
                    if is_left_of {
                        return Vector3::new(-1.0, 0.0, 0.0);
                    }
                    if is_right_of {
                        return Vector3::new(1.0, 0.0, 0.0);
                    }
                }

                Vector3::new(0.0, 0.0, 0.0)
            }
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

            Geometry::AABox { min, max } => {
                let left_t = (min.x - ray.origin.x) / ray.direction().x;
                let right_t = (max.x - ray.origin.x) / ray.direction().x;
                let bottom_t = (min.y - ray.origin.y) / ray.direction().y;
                let top_t = (max.y - ray.origin.y) / ray.direction().y;
                let back_t = (min.z - ray.origin.z) / ray.direction().z;
                let front_t = (max.z - ray.origin.z) / ray.direction().z;

                let t_min = left_t
                    .min(right_t)
                    .max(bottom_t.min(top_t))
                    .max(back_t.min(front_t));

                let t_max = left_t
                    .max(right_t)
                    .min(bottom_t.max(top_t))
                    .min(back_t.max(front_t));

                if t_max < 0.0 {
                    return None;
                }

                // To get here, t_max >= 0
                if t_min > t_max {
                    // Then t_min >= 0 here
                    return None;
                }

                if t_min < 0.0 {
                    return Some(t_max);
                }

                Some(t_min)
            }
        }
    }

    pub fn aabb(&self) -> Geometry {
        match self {
            Geometry::Sphere { center, radius } => Geometry::AABox {
                min: Vector3::new(center.x - radius, center.y - radius, center.z - radius),
                max: Vector3::new(center.x + radius, center.y + radius, center.z + radius),
            },
            Geometry::Triangle { a, b, c } => {
                // TODO: See if this can be done immutably
                let mut xs = [a.x, b.x, c.x];
                xs.sort_by(|a, b| a.total_cmp(b));
                let mut ys = [a.y, b.y, c.y];
                ys.sort_by(|a, b| a.total_cmp(b));
                let mut zs = [a.z, b.z, c.z];
                zs.sort_by(|a, b| a.total_cmp(b));

                Geometry::AABox {
                    min: Vector3::new(xs[0], ys[0], zs[0]),
                    max: Vector3::new(xs[2], ys[2], zs[2]),
                }
            }
            // TODO: Figure out best way to hand this
            Geometry::AABox { min, max } => Geometry::AABox {
                min: *min,
                max: *max,
            },
        }
    }
}
