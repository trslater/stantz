use crate::geometry::Geometry;
use crate::materials::Material;

#[derive(Debug)]
pub struct AABB {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub back: f32,
    pub front: f32,
}

pub trait MakeAABB {
    fn make_aabb(&self) -> AABB;
}

#[derive(Debug)]
pub struct BVH<'a> {
    pub root: BVHNode<'a>,
}

#[derive(Debug)]
// Ideally, object should only be Some when both children are None
pub struct BVHNode<'a> {
    pub aabb: AABB,
    pub object: Option<&'a (Geometry, Material)>,
    pub left_child: Option<Box<Self>>,
    pub right_child: Option<Box<Self>>,
}
