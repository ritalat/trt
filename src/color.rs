use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

use crate::vec3::Color;

pub fn write_color(writer: &mut BufWriter<File>, color: Color, samples: i32) -> io::Result<()> {
    let scale = 1.0 / samples as f64;
    let r = (color.x * scale).sqrt();
    let g = (color.y * scale).sqrt();
    let b = (color.z * scale).sqrt();

    let ir = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i32;

    let tmp = format!("{ir} {ig} {ib}\n");
    writer.write_all(tmp.as_bytes())
}
