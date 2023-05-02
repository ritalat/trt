use crate::ray::Ray;
use crate::vec3::Point;

#[derive(Copy, Clone)]
pub struct Aabb {
    pub min: Point,
    pub max: Point,
}

impl Aabb {
    pub fn new() -> Self {
        Aabb {
            min: Point::new(),
            max: Point::new(),
        }
    }

    pub fn from(a: Point, b: Point) -> Self {
        Aabb { min: a, max: b }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut min = t_min;
        let mut max = t_max;
        for a in 0..3 {
            let t0 =
                ((self.min[a] - r.orig[a]) / r.dir[a]).min((self.max[a] - r.orig[a]) / r.dir[a]);
            let t1 =
                ((self.min[a] - r.orig[a]) / r.dir[a]).max((self.max[a] - r.orig[a]) / r.dir[a]);
            min = t0.max(min);
            max = t1.min(max);
            if max <= min {
                return false;
            }
        }
        true
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Aabb::new()
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = Point::from(
        (box0.min.x).min(box1.min.x),
        (box0.min.y).min(box1.min.y),
        (box0.min.z).min(box1.min.z),
    );
    let big = Point::from(
        (box0.max.x).max(box1.max.x),
        (box0.max.y).max(box1.max.y),
        (box0.max.z).max(box1.max.z),
    );
    Aabb::from(small, big)
}
