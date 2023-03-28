use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use crate::vec3::Vec3;

pub fn write_color(writer: &mut BufWriter<File>, color: &Vec3) -> io::Result<()> {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    let tmp = format!("{ir} {ig} {ib}\n");
    writer.write(tmp.as_bytes())?;
    Ok(())
}
