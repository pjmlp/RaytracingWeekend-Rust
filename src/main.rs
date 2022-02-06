use std::rc::Rc;
use std::io::Error;
use std::f64::consts::PI;
use glam::DVec3;
use image;

use utils::{HitableList, Camera, Sphere, Lambertian, Metal, Dielectric, Color, random_double, write_color_buffer, ray_color};

fn main() -> Result<(), Error> {
    // Image
    const ASPECT_RATIO : f64 = 16.0 / 9.0;
    const IMAGE_WIDTH : u32 = 400;
    const IMAGE_HEIGHT : u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL : i32 = 100;
    const MAX_DEPTH : i32 = 50;
    
    let mut buffer = [0; (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize];

    // World
    let R = (PI / 4.0).cos();
    let mut world = HitableList::new();

    let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Rc::new(Sphere::new(DVec3::new(-R,    0.0, -1.0),  R, Some(material_left))));
    world.add(Rc::new(Sphere::new(DVec3::new( R,    0.0, -1.0),  R, Some(material_right))));

    // Camera
    let cam = Camera::new(90.0, ASPECT_RATIO);

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
