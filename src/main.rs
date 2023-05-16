use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::rc::Rc;

use crate::aarect::Xyrect;
use crate::camera::Camera;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::moving_sphere::MovingSphere;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point, Vec3};

mod aabb;
mod aarect;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod ray;
mod sphere;
mod vec3;

#[allow(dead_code)]
fn random_scene() -> HittableList {
    let mut objects = HittableList::new();
    let mut rng = thread_rng();

    let ground_material = Rc::new(Lambertian::from(Color::from(0.8, 0.8, 0.0)));
    objects.push(Rc::new(Sphere::from(
        Point::from(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Point::from(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (center - Point::from(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::from(albedo));
                    let center2 = center + Vec3::from(0.0, rng.gen_range(0.0..0.5), 0.0);
                    objects.push(Rc::new(MovingSphere::from(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Rc::new(Metal::from(albedo, fuzz));
                    objects.push(Rc::new(Sphere::from(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Rc::new(Dielectric::from(1.5));
                    objects.push(Rc::new(Sphere::from(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::from(1.5));
    objects.push(Rc::new(Sphere::from(
        Point::from(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::from(Color::from(0.4, 0.2, 0.1)));
    objects.push(Rc::new(Sphere::from(
        Point::from(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::from(Color::from(0.7, 0.6, 0.5), 0.0));
    objects.push(Rc::new(Sphere::from(
        Point::from(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    objects
}

#[allow(dead_code)]
fn simple_ligth_scene() -> HittableList {
    let mut objects = HittableList::new();

    let ground_material = Rc::new(Lambertian::from(Color::from(0.8, 0.8, 0.0)));
    objects.push(Rc::new(Sphere::from(
        Point::from(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let sphere_material = Rc::new(Metal::from(Color::from(0.8, 0.8, 0.8), 0.8));
    objects.push(Rc::new(Sphere::from(
        Point::from(0.0, 2.0, 0.0),
        2.0,
        sphere_material,
    )));

    // Note that the light is brighter than (1, 1, 1)
    let light_material = Rc::new(DiffuseLight::from(Color::from(4.0, 4.0, 4.0)));
    objects.push(Rc::new(Xyrect::from(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        light_material,
    )));

    objects
}

fn ray_color(r: &Ray, background: Color, objects: &mut dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new();
    }

    let record = objects.hit(r, 0.001, f64::INFINITY);

    match record {
        Some(rec) => match rec.mat.scatter(r, &rec) {
            Some((attenuation, scattered)) => {
                rec.mat.emitted()
                    + attenuation * ray_color(&scattered, background, objects, depth - 1)
            }
            None => rec.mat.emitted(),
        },
        // If the ray hits nothing, return the background color
        None => background,
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
    let mut objects = simple_ligth_scene();
    let background = Color::new();

    let lookfrom = Point::from(26.0, 3.0, 6.0);
    let lookat = Point::from(0.0, 2.0, 0.0);
    let vup = Vec3::from(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::from(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

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
                pixel_color = pixel_color + ray_color(&r, background, &mut objects, max_depth);
            }
            write_color(&mut writer, pixel_color, samples_per_pixel)?;
        }
    }
    eprintln!("\nDone!");

    Ok(())
}
