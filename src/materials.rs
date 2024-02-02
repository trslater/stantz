use crate::lighting::Color;

pub struct Material {
    pub diffusion: f32,
    pub specularity: f32,
    pub shininess: i32,
    pub reflectance: f32,
    pub color: Color,
}
