use std::rc::Rc;
use glam::DVec3;

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

mod color;
pub use color::*;

// remaining crate code

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

pub fn ray_color(r: Ray, world : &dyn Hitable, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.0, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + random_unit_vector();
        return 0.5 * ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1);
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

pub fn random_dvec3() -> DVec3 {
    // Returns a random vector in [0,1).
    DVec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>())
}

pub fn random_dvec3_with(min: f64, max: f64) -> DVec3 {
    // Returns a random vector in [min,max).
    DVec3::new(random_double_with(min, max), random_double_with(min, max), random_double_with(min, max))
}

pub fn random_in_unit_sphere() -> DVec3 {
    // Returns a random vector in [0,1).
    loop {
        let p = random_dvec3();
        if p.length_squared() >= 1.0 {
            continue;
        }

        return p;
    }    
}

pub fn random_unit_vector() -> DVec3 {
    random_in_unit_sphere().normalize()
}
