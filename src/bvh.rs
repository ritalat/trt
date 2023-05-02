use rand::prelude::*;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::aabb::{surrounding_box, Aabb};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;

pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub bbox: Aabb,
}

impl BvhNode {
    pub fn from(
        src_objects: &[Rc<dyn Hittable>],
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        // Create a modifiable array of the source scene objects
        let mut objects = src_objects.to_owned();
        let mut rng = thread_rng();
        let axis = rng.gen_range(0..=2);

        let comparator = |a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>| bbox_compare(a, b, axis);

        let object_span = end - start;

        let left;
        let right;

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]).is_lt() {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects.sort_by(comparator);
            let mid = start + object_span / 2;
            left = Rc::new(BvhNode::from(&objects, start, mid, time0, time1));
            right = Rc::new(BvhNode::from(&objects, mid, end, time0, time1));
        }

        let bbox_left = left.bounding_box(time0, time1);
        let bbox_right = right.bounding_box(time0, time1);
        let bbox = surrounding_box(
            bbox_left.unwrap_or_default(),
            bbox_right.unwrap_or_default(),
        );

        BvhNode { left, right, bbox }
    }

    pub fn from_list(list: &HittableList, time0: f64, time1: f64) -> Self {
        Self::from(&list.objects, 0, list.objects.len(), time0, time1)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }

        let rec_left = self.left.hit(r, t_min, t_max);
        let t = match &rec_left {
            Some(rec) => rec.t,
            None => t_max,
        };
        let rec_right = self.right.hit(r, t_min, t);

        match rec_right {
            Some(rec) => Some(rec),
            None => rec_left,
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}

fn bbox_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: i32) -> Ordering {
    let a_bbox = a.bounding_box(0.0, 0.0);
    let b_bbox = b.bounding_box(0.0, 0.0);
    let a_val = match a_bbox {
        Some(bbox) => bbox.min[axis as usize],
        None => 0.0,
    };
    let b_val = match b_bbox {
        Some(bbox) => bbox.min[axis as usize],
        None => 0.0,
    };
    a_val.partial_cmp(&b_val).unwrap()
}
