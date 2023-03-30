use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

use crate::vec3::Color;

pub fn write_color(writer: &mut BufWriter<File>, color: &Color) -> io::Result<()> {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    let tmp = format!("{ir} {ig} {ib}\n");
    writer.write_all(tmp.as_bytes())?;
    Ok(())
}
