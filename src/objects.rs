use crate::{geometry::Geometry, materials::Material};

pub struct Object<'a> {
    pub mesh: &'a Vec<Geometry>,
    pub material: &'a Material,
}

pub struct ObjectIterator<'a> {
    entries: Vec<(&'a Geometry, &'a Material)>,
    index: usize,
}

impl<'a> ObjectIterator<'a> {
    fn new(object: &'a Object) -> Self {
        ObjectIterator {
            entries: object
                .mesh
                .iter()
                .map(|geometry| (geometry, object.material))
                .collect(),
            index: 0,
        }
    }
}

impl<'a> Iterator for ObjectIterator<'a> {
    type Item = (&'a Geometry, &'a Material);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entries.len() {
            let entry = self.entries[self.index];
            self.index += 1;
            Some(entry)
        } else {
            None
        }
    }
}

impl Object<'_> {
    pub fn iter(&self) -> ObjectIterator {
        ObjectIterator::new(self)
    }
}
