use std::rc::Rc;

use crate::aabb::{surrounding_box, Aabb};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit = t_max;
        let mut record = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_hit) {
                closest_hit = rec.t;
                record = Some(rec);
            }
        }
        record
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }

        let mut out_box: Option<Aabb> = None;
        let mut first_box = true;

        for object in &self.objects {
            let tmp_box = object.bounding_box(time0, time1);
            match tmp_box {
                Some(bbox) => {
                    out_box = if first_box {
                        first_box = false;
                        Some(bbox)
                    } else {
                        Some(surrounding_box(out_box.unwrap(), bbox))
                    }
                }
                None => return None,
            }
        }

        out_box
    }
}
