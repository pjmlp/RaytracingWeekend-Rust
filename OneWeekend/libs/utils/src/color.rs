use glam::DVec3;
use std::io::{Stdout, Write};
use std::io::Error;

pub fn write_color(out : Stdout, pixel_color : DVec3, samples_per_pixel:i32) -> Result<(), Error>  {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / (samples_per_pixel as f64);
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    let mut handle = out.lock();

    let ir = (256.0 * r.clamp(0.0, 0.999)) as i64;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i64;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i64;

    handle.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;

    Ok(())
}
