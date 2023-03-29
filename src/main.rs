use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use crate::vec3::{Vec3, Color, Point};
use crate::color::write_color;
use crate::ray::Ray;

mod vec3;
mod color;
mod ray;

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> bool {
    let oc = r.orig - *center;
    let a = vec3::dot(&r.dir, &r.dir);
    let b = 2.0 * vec3::dot(&oc, &r.dir);
    let c = vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point::from(0.0, 0.0, -1.0), 0.5, r) {
        return Color::from(1.0, 0.0, 0.0);
    }
    let unit_direction = vec3::unit_vector(r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_lenght = 1.0;

    let origin = Point::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_lenght);

    // Render
    let f = File::create("img.ppm")?;
    let mut writer = BufWriter::new(f);

    let header = format!("P3\n{image_width} {image_height}\n255\n");
    writer.write(header.as_bytes())?;

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::from(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let color = ray_color(&r);
            write_color(&mut writer, &color)?;
        }
    }
    println!("\nDone!");

    Ok(())
}
