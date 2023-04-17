use crate::ray::Ray;
use crate::vec3::{self, Point, Vec3};

pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn from(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vec3::cross(&vup, &w));
        let v = vec3::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::from(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
