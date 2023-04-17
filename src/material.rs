use rand::prelude::*;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{self, Color, Vec3};

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new() -> Self {
        Lambertian {
            albedo: Color::new(),
        }
    }

    pub fn from(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some((self.albedo, Ray::from(rec.p, scatter_direction)))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new() -> Self {
        Metal {
            albedo: Color::new(),
            fuzz: 1.0,
        }
    }

    pub fn from(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: match fuzz < 1.0 {
                true => fuzz,
                false => 1.0,
            },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = vec3::reflect(vec3::unit_vector(r.dir), rec.normal);
        let scattered = Ray::from(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;

        match vec3::dot(&scattered.dir, &rec.normal) > 0.0 {
            true => Some((attenuation, scattered)),
            false => None,
        }
    }
}

pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new() -> Self {
        Dielectric { ir: 1.5 } // Glass
    }

    pub fn from(ir: f64) -> Self {
        Dielectric { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::from(1.0, 1.0, 1.0);
        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };

        let unit_direction = vec3::unit_vector(r.dir);
        let cos_theta = vec3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                vec3::reflect(unit_direction, rec.normal)
            } else {
                vec3::refract(unit_direction, rec.normal, refraction_ratio)
            };

        let scattered = Ray::from(rec.p, direction);
        Some((attenuation, scattered))
    }
}
