use crate::objects::Object;

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

#[derive(Debug)]
pub struct BVH<'a> {
    pub root: BVHNode<'a>,
}

#[derive(Debug)]
// Ideally, object should only be Some when both children are None
pub struct BVHNode<'a> {
    pub aabb: AABB,
    pub object: Option<&'a Object>,
    pub left_child: Option<Box<Self>>,
    pub right_child: Option<Box<Self>>,
}
