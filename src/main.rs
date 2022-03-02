use std::io::Error;
use glam::DVec3;
use image;

mod utils;

use utils::{Camera, Point3, random_scene, random_double, write_color_buffer, ray_color};

fn main() -> Result<(), Error> {
    // Image
    const ASPECT_RATIO : f64 = 3.0 / 2.0;
    const IMAGE_WIDTH : u32 = 1200;
    const IMAGE_HEIGHT : u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL : i32 = 500;
    const MAX_DEPTH : i32 = 50;
    
    let mut buffer = vec![0; (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize];

    // World
    let world = random_scene();

    // Camera
    let  lookfrom = Point3::new(13.0, 2.0, 3.0);
    let  lookat = Point3::new(0.0, 0.0, 0.0);
    let  vup = DVec3::new(0.0, 1.0, 0.0);
    
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    
    let cam = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);    

    // Render   
    let mut current = 0;
    
    for j in (0 .. IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", j);

        for i in 0 .. IMAGE_WIDTH {
            let mut pixel_color = DVec3::new(0.0, 0.0, 0.0);

            for _ in 0 .. SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH-1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            
            write_color_buffer(&mut buffer, current, pixel_color, SAMPLES_PER_PIXEL);
            current += 3;
        }
    }
    
    
    
    // Save the buffer as "image.png"
    image::save_buffer("image.png", &buffer, IMAGE_WIDTH, IMAGE_HEIGHT, image::ColorType::Rgb8).unwrap();

    eprintln!("\nDone");
    Ok(())
}
