use na::Vector3;

pub struct Camera {
    pub origin: Vector3<f32>,
    pub fov: f32,
    pub focal_length: f32,
}

impl Camera {
    pub fn focal_plane_height(&self) -> f32 {
        let fov_radians = self.fov * std::f32::consts::PI / 180.0;

        (fov_radians / 2.0).tan() * 2.0 * self.focal_length
    }

    pub fn focal_plane_z(&self) -> f32 {
        self.origin.z - self.focal_length
    }
}
