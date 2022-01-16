use std::rc::Rc;
use std::io::Error;
use glam::DVec3;
use image;

use utils::{HitableList, Camera, Sphere, random_double, write_color_buffer, ray_color};

fn main() -> Result<(), Error> {
    // Image
    const ASPECT_RATIO : f32 = 16.0 / 9.0;
    const IMAGE_WIDTH : u32 = 400;
    const IMAGE_HEIGHT : u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL : i32 = 100;
    const MAX_DEPTH : i32 = 50;
    
    let mut buffer = [0; (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize];

    // World
    let mut world = HitableList::new();

    world.add(Rc::new(Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0)));
    
    // Camera
    let cam = Camera::new();

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
