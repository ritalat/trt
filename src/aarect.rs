use std::rc::Rc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct Xyrect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mat: Rc<dyn Material>,
}

impl Xyrect {
    #[allow(dead_code)]
    pub fn from(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Rc<dyn Material>) -> Self {
        Xyrect {
            x0,
            x1,
            y0,
            y1,
            k,
            mat,
        }
    }
}

impl Hittable for Xyrect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let p = r.at(t);
        let outward_normal = Vec3::from(0.0, 0.0, 1.0);
        Some(HitRecord::from(r, p, t, outward_normal, self.mat.clone()))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the z
        // dimension a small amount
        Some(Aabb::from(
            Point::from(self.x0, self.y0, self.k - 0.0001),
            Point::from(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
