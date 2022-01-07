use glam::DVec3;
use std::io::{Stdout, Write};
use std::io::Error;

pub fn write_color(out : Stdout, pixel_color : DVec3) -> Result<(), Error>  {
    let ir = (255.999 * pixel_color.x) as i64;
    let ig = (255.999 * pixel_color.y) as i64;
    let ib = (255.999 * pixel_color.z) as i64;

    let mut handle = out.lock();

    handle.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;

    Ok(())
}
