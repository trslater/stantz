use na::Vector3;

use crate::geometry::{aa_box::AABoxGeometry, Center, NormalAt, AABB};

#[derive(Debug, Clone)]
pub struct TriangleGeometry {
    pub a: Vector3<f32>,
    pub b: Vector3<f32>,
    pub c: Vector3<f32>,
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

impl NormalAt for TriangleGeometry {
    fn normal_at(&self, _: &Vector3<f32>) -> Vector3<f32> {
        (self.b - self.a).cross(&(self.c - self.a)).normalize()
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
