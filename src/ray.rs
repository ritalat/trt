use crate::vec3::{Point, Vec3};

#[derive(Debug)]
pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
    pub t: f64,
}

impl Ray {
    pub fn from(origin: Point, direction: Vec3, time: f64) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            t: time,
        }
    }

    pub fn at(&self, t: f64) -> Point {
        self.orig + t * self.dir
    }
}
