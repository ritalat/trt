use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::rc::Rc;

use crate::camera::Camera;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point};

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

fn ray_color(r: &Ray, objects: &mut Vec<Box<dyn Hittable>>, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new();
    }

    let mut closest_hit = f64::INFINITY;
    let mut record = None;

    for object in &mut *objects {
        if let Some(rec) = object.hit(r, 0.001, closest_hit) {
            closest_hit = rec.t;
            record = Some(rec);
        }
    }

    match record {
        Some(rec) => match rec.mat.scatter(r, &rec) {
            Some((attenuation, scattered)) => {
                attenuation * ray_color(&scattered, objects, depth - 1)
            }
            None => Color::new(),
        },
        None => {
            let unit_direction = vec3::unit_vector(r.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
        }
    }
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground = Rc::new(Lambertian::from(Color::from(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::from(Color::from(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::from(1.5));
    let material_right = Rc::new(Metal::from(Color::from(0.8, 0.6, 0.2), 0.0));

    objects.push(Box::new(Sphere::from(
        Point::from(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    objects.push(Box::new(Sphere::from(
        Point::from(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    objects.push(Box::new(Sphere::from(
        Point::from(-1.0, -0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    objects.push(Box::new(Sphere::from(
        Point::from(-1.0, -0.0, -1.0),
        -0.4,
        material_left,
    )));
    objects.push(Box::new(Sphere::from(
        Point::from(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let camera = Camera::new();

    // Render
    let f = File::create("img.ppm")?;
    let mut writer = BufWriter::new(f);

    let header = format!("P3\n{image_width} {image_height}\n255\n");
    writer.write_all(header.as_bytes())?;

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j:03}");
        for i in 0..image_width {
            let mut pixel_color = Color::new();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &mut objects, max_depth);
            }
            write_color(&mut writer, pixel_color, samples_per_pixel)?;
        }
    }
    eprintln!("\nDone!");

    Ok(())
}
