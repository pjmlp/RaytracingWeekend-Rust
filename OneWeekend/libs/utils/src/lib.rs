use std::rc::Rc;
use glam::DVec3;
use std::io::{Stdout, Write};
use std::io::Error;

// republish the internal modules
mod algebra;
pub use algebra::*;

mod ray;
pub use ray::*;

mod sphere;
pub use sphere::*;

mod hitable;
pub use hitable::*;

mod camera;
pub use camera::*;

// remaining crate code

pub fn write_color(out : Stdout, pixel_color : DVec3, samples_per_pixel:i32) -> Result<(), Error>  {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

// Divide the color by the number of samples.
    let scale = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    let mut handle = out.lock();

    let ir = (256.0 * r.clamp(0.0, 0.999)) as i64;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i64;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i64;

    handle.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;

    Ok(())
}

pub fn hit_sphere(center : DVec3, radius: f64, r : &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

pub fn ray_color(r: Ray, world : &dyn Hitable) -> Color {
    let mut rec = HitRecord::default();

    if world.hit(&r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0,1.0,1.0));
    }
    
    let unit_direction = unit_vector(r.direction());
    let t = 0.5*(unit_direction.y + 1.0);
    
    (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
}


pub struct HitableList {
    objects : Vec<Rc<dyn Hitable>>
}

impl HitableList {
    pub fn new() -> Self {
        HitableList { objects: Vec::new() }
    }


    pub fn copy_from(other:Vec<Rc<dyn Hitable>>) -> Self {
        HitableList { objects: other }
    }

    pub fn clear(mut self) {
        self.objects.clear();
    }
    
    pub fn add(&mut self, obj:Rc<dyn Hitable>) {
        self.objects.push(obj)
    }    
}

impl Hitable for HitableList {
 
    fn hit(&self, ray: &Ray, t_min:f64, t_max:f64, rec : &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut closest_so_far = t_max;
        let mut hit_anything = false;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}

pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    rand::random::<f64>()
}

pub fn random_double_with(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max-min)*random_double()
}
