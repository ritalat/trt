use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

use crate::color::write_color;
use crate::ray::Ray;
use crate::vec3::{Color, Point, Vec3};

mod color;
mod ray;
mod vec3;

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> f64 {
    let oc = r.orig - *center;
    let a = r.dir.length_squared();
    let half_b = vec3::dot(&oc, &r.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let mut t = hit_sphere(&Point::from(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = vec3::unit_vector(r.at(t) - Vec3::from(0.0, 0.0, -1.0));
        return 0.5 * Color::from(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = vec3::unit_vector(r.dir);
    t = 0.5 * (unit_direction.y + 1.0);
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
    let focal_length = 1.0;

    let origin = Point::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

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
            let r = Ray::from(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = ray_color(&r);
            write_color(&mut writer, &color)?;
        }
    }
    println!("\nDone!");

    Ok(())
}
