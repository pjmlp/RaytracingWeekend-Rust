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

mod material;
pub use material::*;

mod scene;
pub use scene::*;

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
        return Color::ZERO;
    }

    if world.hit(&r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        if let Some(mat) = &rec.mat_ptr  {
            // not using && due to the experimental warning
            if mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(scattered, world, depth-1);
            }
        }
        
        return Color::ZERO;
    }
    
    let unit_direction = unit_vector(r.direction());
    let t = 0.5*(unit_direction.y + 1.0);
    
    (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
}
