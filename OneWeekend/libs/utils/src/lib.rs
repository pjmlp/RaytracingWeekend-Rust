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


// remaining crate code

pub fn write_color(out : Stdout, pixel_color : DVec3) -> Result<(), Error>  {
    let ir = (255.999 * pixel_color.x) as i64;
    let ig = (255.999 * pixel_color.y) as i64;
    let ib = (255.999 * pixel_color.z) as i64;

    let mut handle = out.lock();

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
