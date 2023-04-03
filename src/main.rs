use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

use crate::camera::Camera;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point};

mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

fn ray_color(r: &Ray, objects: &mut Vec<Box<dyn Hittable>>) -> Color {
    let mut closest_hit = f64::INFINITY;
    let mut record = None;

    for object in objects {
        if let Some(rec) = object.hit(r, 0.0, closest_hit) {
            closest_hit = rec.t;
            record = Some(rec);
        }
    }

    match record {
        Some(rec) => 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0)),
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

    // World
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere::from(Point::from(0.0, 0.0, -1.0), 0.5)));
    objects.push(Box::new(Sphere::from(
        Point::from(0.0, -100.5, -1.0),
        100.0,
    )));

    let camera = Camera::new();

    // Render
    let f = File::create("img.ppm")?;
    let mut writer = BufWriter::new(f);

    let header = format!("P3\n{image_width} {image_height}\n255\n");
    writer.write_all(header.as_bytes())?;

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = camera.get_ray(u, v);
            let color = ray_color(&r, &mut objects);
            write_color(&mut writer, &color)?;
        }
    }
    println!("\nDone!");

    Ok(())
}
