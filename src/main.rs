use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use crate::vec3::Vec3;
use crate::color::write_color;

mod vec3;
mod color;

fn main() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let f = File::create("img.ppm")?;
    let mut writer = BufWriter::new(f);

    let header = format!("P3\n{image_width} {image_height}\n255\n");
    writer.write(header.as_bytes())?;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let color = Vec3 {
                x: i as f64 / image_width as f64,
                y: j as f64 / image_height as f64,
                z: 0.25,
            };
            write_color(&mut writer, &color)?;
        }
    }

    Ok(())
}
