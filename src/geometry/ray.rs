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
