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
