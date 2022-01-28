use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use super::algebra::*;
use super::ray::*;

use glam::DVec3;

pub struct HitRecord {
    pub p : Point3,
    pub normal : DVec3,
    pub mat_ptr : Option<Rc<dyn Material>>,
    pub t : f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &DVec3) {
        self.front_face = r.direction().dot(*outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::ZERO,
            normal: DVec3::ZERO,
            mat_ptr: None,
            t : 0.0,
            front_face: false
        } 
    }
}

impl Clone for HitRecord {
    fn clone(&self) -> HitRecord {
        HitRecord {
            p: self.p,
            normal: self.normal,
            mat_ptr: self.mat_ptr.clone(),
            t : self.t,
            front_face: self.front_face
        } 
    }
}

impl Debug for HitRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HitRecord")
         .field("p", &self.p)
         .field("normal", &self.normal)
         .field("t", &self.t)
         .field("front_face", &self.front_face)
         .finish()
    } 
}

/// What any object being rendered needs to support to validate ray intersections.
pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min:f64, t_max:f64, rec : &mut HitRecord) -> bool;
}

/// What different kinds of materials need to implement
pub trait Material {
    fn scatter(&self, ray: &Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool;
}