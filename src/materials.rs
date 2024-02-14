use crate::lighting::Color;

#[derive(Debug)]
pub struct Material {
    pub diffused: f32,
    pub specular: f32,
    pub shininess: i32,
    pub reflectance: f32,
    pub color: Color,
}
