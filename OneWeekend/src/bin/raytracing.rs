use std::rc::Rc;
use glam::DVec3;
use utils::{HitableList};

fn main() {
    // Image
    const ASPECT_RATIO : f32 = 16.0 / 9.0;
    const IMAGE_WIDTH : i32 = 400;
    const IMAGE_HEIGHT : i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL : i32 = 100;

    // World
    let mut world = HitableList::new();

    world.add(Rc::new(utils::Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(utils::Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0)));
    
    // Camera
    let viewport_height = 2.0f64;
    let viewport_width = (ASPECT_RATIO as f64) * viewport_height;
    let focal_length = 1.0;

    let origin = utils::Point3::ZERO;
    let horizontal = DVec3::new(viewport_width, 0.0, 0.0);
    let vertical = DVec3::new (0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - DVec3::new(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    
    for j in (0 .. IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", j);

        for i in 0 .. IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            
            let r = utils::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = utils::ray_color(r, &world);

            utils::write_color(std::io::stdout(), pixel_color, SAMPLES_PER_PIXEL).expect("Failed ot write pixel");
        }
    }

    eprintln!("\nDone");
}
