use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: &Vec3, dir: &Vec3) -> Self {
        Ray {
            orig: orig.clone(),
            dir: dir.normalize(),
        }
    }
}