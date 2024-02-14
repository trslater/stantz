use crate::lighting::Color;

#[derive(Debug)]
pub struct Material {
    pub ambient_color: Color,
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub shininess: i32,
    pub reflectance: f32,
}
