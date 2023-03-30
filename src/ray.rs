use crate::vec3::{Point, Vec3};

#[derive(Debug)]
pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn from(origin: Point, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn at(&self, t: f64) -> Point {
        self.orig + t * self.dir
    }
}
