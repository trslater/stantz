use na::{Matrix3, Vector3};

use crate::rendering::ray::Ray;

use super::{aa_box::AABoxGeometry, Center, Intersection, NormalAt, AABB};

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

impl Intersection<Ray, f32> for TriangleGeometry {
    type Argument = Ray;
    type Output = f32;

    fn intersection(&self, other: &Self::Argument) -> Option<Self::Output> {
        Matrix3::from_columns(&[self.b - self.a, self.c - self.a, -other.direction()])
            .lu()
            .solve(&(other.origin - self.a))
            .map(|uvt| [uvt[0], uvt[1], uvt[2]])
            .filter(|[u, v, ..]| {
                let w = 1.0 - u - v;

                u >= &0.0 && u <= &1.0 && v >= &0.0 && v <= &1.0 && w >= 0.0 && w <= 1.0
            })
            .map(|[.., t]| t)
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
