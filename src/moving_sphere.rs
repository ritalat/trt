use std::rc::Rc;

use crate::aabb::{surrounding_box, Aabb};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point};

pub struct MovingSphere {
    center0: Point,
    center1: Point,
    time0: f64,
    time1: f64,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl MovingSphere {
    #[allow(dead_code)]
    pub fn from(
        center0: Point,
        center1: Point,
        time0: f64,
        time1: f64,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat,
        }
    }

    pub fn center(&self, time: f64) -> Point {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center(r.t);
        let a = r.dir.length_squared();
        let half_b = vec3::dot(&oc, &r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center(r.t)) / self.radius;

        Some(HitRecord::from(r, p, t, outward_normal, self.mat.clone()))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb::from(
            self.center(time0) - Point::from(self.radius, self.radius, self.radius),
            self.center(time0) + Point::from(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::from(
            self.center(time1) - Point::from(self.radius, self.radius, self.radius),
            self.center(time1) + Point::from(self.radius, self.radius, self.radius),
        );
        Some(surrounding_box(box0, box1))
    }
}
