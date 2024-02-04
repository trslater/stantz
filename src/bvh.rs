#[derive(Debug)]
pub struct AABB {
    pub x_lower: f32,
    pub x_upper: f32,
    pub y_lower: f32,
    pub y_upper: f32,
    pub z_lower: f32,
    pub z_upper: f32,
}

pub trait MakeAABB {
    fn make_aabb(&self) -> AABB;
}
