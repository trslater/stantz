use na::Vector3;

use crate::geometry::{ray::Ray, Center, Intersection, NormalAt, AABB};

#[derive(Debug, Clone)]
pub struct AABoxGeometry {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
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

impl AABB for AABoxGeometry {
    fn aabb(&self) -> AABoxGeometry {
        self.clone()
    }
}
