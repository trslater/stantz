use crate::geometry::Geometry;
use crate::materials::Material;

#[derive(Debug)]
pub struct BVH<'a> {
    pub root: BVHNode<'a>,
}

#[derive(Debug)]
// Ideally, object should only be Some when both children are None
pub struct BVHNode<'a> {
    pub object: Option<&'a (Geometry, Material)>,
    pub left_child: Option<Box<Self>>,
    pub right_child: Option<Box<Self>>,
}
