pub mod ray;

use na::{geometry, Matrix3, Vector3};
use ray::Ray;

#[derive(Debug, Clone)]
pub enum Geometry {
    AABox(AABoxGeometry),
    Sphere(SphereGeometry),
    Triangle(TriangleGeometry),
}

#[derive(Debug, Clone)]
pub struct AABoxGeometry {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct SphereGeometry {
    pub center: Vector3<f32>,
    pub radius: f32,
}

#[derive(Debug, Clone)]
pub struct TriangleGeometry {
    pub a: Vector3<f32>,
    pub b: Vector3<f32>,
    pub c: Vector3<f32>,
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

impl Center for AABoxGeometry {
    fn center(&self) -> Vector3<f32> {
        Vector3::new(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }
}

impl Center for SphereGeometry {
    fn center(&self) -> Vector3<f32> {
        self.center
    }
}

impl Center for TriangleGeometry {
    fn center(&self) -> Vector3<f32> {
        Vector3::new(
            [self.a.x, self.b.x, self.c.x].iter().sum::<f32>() / 3.0,
            [self.a.y, self.b.y, self.c.y].iter().sum::<f32>() / 3.0,
            [self.a.z, self.b.z, self.c.z].iter().sum::<f32>() / 3.0,
        )
    }
}

impl NormalAt for AABoxGeometry {
    fn normal_at(&self, point: &Vector3<f32>) -> Vector3<f32> {
        // TODO: Implement edge/corner cases (literally)
        let is_left_of = point.x <= self.min.x;
        let is_right_of = point.x >= self.max.x;
        let is_below = point.y <= self.min.y;
        let is_above = point.y >= self.max.y;
        let is_behind = point.z <= self.min.z;
        let is_in_front_of = point.z >= self.max.z;

        let is_in_x_bounds = (point.x >= self.min.x && point.x <= self.min.x + 0.1)
            || (point.x <= self.max.x && point.x >= self.max.x - 0.1);
        let is_in_y_bounds = (point.y >= self.min.y && point.y <= self.min.y + 0.1)
            || (point.y <= self.max.y && point.y >= self.max.y - 0.1);
        let is_in_z_bounds = (point.z >= self.min.z && point.z <= self.min.z + 0.1)
            || (point.z <= self.max.z && point.z >= self.max.z - 0.1);

        if is_in_x_bounds || is_in_y_bounds {
            if is_behind {
                return Vector3::new(0.0, 0.0, -1.0);
            }
            if is_in_front_of {
                return Vector3::new(0.0, 0.0, 1.0);
            }
        }
        if is_in_x_bounds || is_in_z_bounds {
            if is_below {
                return Vector3::new(0.0, -1.0, 0.0);
            }
            if is_above {
                return Vector3::new(0.0, 1.0, 0.0);
            }
        }
        if is_in_y_bounds || is_in_z_bounds {
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

impl NormalAt for SphereGeometry {
    fn normal_at(&self, point: &Vector3<f32>) -> Vector3<f32> {
        (point - self.center).normalize()
    }
}

impl NormalAt for TriangleGeometry {
    fn normal_at(&self, _: &Vector3<f32>) -> Vector3<f32> {
        (self.b - self.a).cross(&(self.c - self.a)).normalize()
    }
}

impl Intersection for AABoxGeometry {
    fn intersection(&self, ray: &Ray) -> Option<f32> {
        let left_t = (self.min.x - ray.origin.x) / ray.direction().x;
        let right_t: f32 = (self.max.x - ray.origin.x) / ray.direction().x;
        let bottom_t = (self.min.y - ray.origin.y) / ray.direction().y;
        let top_t = (self.max.y - ray.origin.y) / ray.direction().y;
        let back_t = (self.min.z - ray.origin.z) / ray.direction().z;
        let front_t = (self.max.z - ray.origin.z) / ray.direction().z;

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

impl Intersection for TriangleGeometry {
    fn intersection(&self, ray: &Ray) -> Option<f32> {
        Matrix3::from_columns(&[self.b - self.a, self.c - self.a, -ray.direction()])
            .lu()
            .solve(&(ray.origin - self.a))
            .map(|uvt| [uvt[0], uvt[1], uvt[2]])
            .filter(|[u, v, ..]| {
                let w: f32 = 1.0 - u - v;

                u >= &0.0 && u <= &1.0 && v >= &0.0 && v <= &1.0 && w >= 0.0 && w <= 1.0
            })
            .map(|[.., t]| t)
    }
}

impl AABB for AABoxGeometry {
    fn aabb(&self) -> AABoxGeometry {
        self.clone()
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

impl AABB for TriangleGeometry {
    fn aabb(&self) -> AABoxGeometry {
        // TODO: See if this can be done immutably
        let mut xs = [self.a.x, self.b.x, self.c.x];
        xs.sort_by(|p, q| p.total_cmp(q));
        let mut ys = [self.a.y, self.b.y, self.c.y];
        ys.sort_by(|p, q| p.total_cmp(q));
        let mut zs = [self.a.z, self.b.z, self.c.z];
        zs.sort_by(|p, q| p.total_cmp(q));

        AABoxGeometry {
            min: Vector3::new(xs[0], ys[0], zs[0]),
            max: Vector3::new(xs[2], ys[2], zs[2]),
        }
    }
}
