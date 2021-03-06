use std::rc::Rc;
use super::algebra::*;
use super::hitable::*;
use super::ray::*;

/// Sphere objects used in the scene
pub struct Sphere {
    center : Point3,
    radius : f64,
    pub mat_ptr : Option<Rc<dyn Material>>,
}


impl Sphere {
    /// Creates a sphere located at the specific center with the given radius
    pub fn new(cen : Point3, r:f64, m : Option<Rc<dyn Material>>) -> Self {
        Sphere{center:cen, radius:r, mat_ptr: m}
    }
}

/// Spheres are Hitable objects, so validate if the light rays hit them on scene
impl Hitable for Sphere {

    fn hit(&self, r: &Ray, t_min:f64, t_max:f64, rec : &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false
        } 

         // Find the nearest root that lies in the acceptable range.
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }
}
