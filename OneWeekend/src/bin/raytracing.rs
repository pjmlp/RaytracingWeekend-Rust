use std::rc::Rc;
use glam::DVec3;

use utils::{HitableList, Camera, random_double};

fn main() {
    // Image
    const ASPECT_RATIO : f32 = 16.0 / 9.0;
    const IMAGE_WIDTH : i32 = 400;
    const IMAGE_HEIGHT : i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL : i32 = 100;
    const MAX_DEPTH : i32 = 50;

    // World
    let mut world = HitableList::new();

    world.add(Rc::new(utils::Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(utils::Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0)));
    
    // Camera
    let cam = Camera::new();

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    
    for j in (0 .. IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", j);

        for i in 0 .. IMAGE_WIDTH {
            let mut pixel_color = DVec3::new(0.0, 0.0, 0.0);

            for _ in 0 .. SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH-1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += utils::ray_color(r, &world, MAX_DEPTH);
            }


            utils::write_color(std::io::stdout(), pixel_color, SAMPLES_PER_PIXEL).expect("Failed ot write pixel");
        }
    }

    eprintln!("\nDone");
}
