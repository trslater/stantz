use na::Vector3;

use crate::{
    geometry::{Geometry, Intersection, NormalAt},
    lighting::{Color, Light},
    materials::Material,
};

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

    pub fn color(&self, entries: &Vec<(&Geometry, &Material)>, lights: &Vec<Light>) -> Color {
        entries
            .iter()
            .filter_map(|(geometry, material)| {
                geometry
                    .intersection(self)
                    .and_then(|t| Some((t, geometry, material)))
            })
            // TODO: Does defaulting to less make sense?
            .min_by(|(ta, ..), (tb, ..)| ta.total_cmp(tb))
            .map(|(t, geometry, material)| self.color_at(t, geometry, material, lights))
            .unwrap_or(Color::new_black())
    }

    fn color_at(
        &self,
        t: f32,
        geometry: &Geometry,
        material: &Material,
        lights: &Vec<Light>,
    ) -> Color {
        let hit_point = self.point_at(t);
        let hit_normal = geometry.normal_at(&hit_point);

        lights
            .iter()
            .map(|light| {
                let light_direction = light.direction_from(&hit_point);

                // TODO: hit_point used twice. Can we optimize?
                let diffusion = light_direction.dot(&hit_normal);
                let specularity = (light_direction - self.direction())
                    .normalize()
                    .dot(&hit_normal);

                light.color
                    * material.color
                    * (material.diffused * diffusion
                        + material.specular * specularity.powi(material.shininess))
            })
            .sum()
    }
}
