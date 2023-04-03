use crate::ray::Ray;
use crate::vec3::{self, Point, Vec3};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn from(r: &Ray, p: Point, t: f64, outward_normal: Vec3) -> Self {
        let front_face = vec3::dot(&r.dir, &outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
