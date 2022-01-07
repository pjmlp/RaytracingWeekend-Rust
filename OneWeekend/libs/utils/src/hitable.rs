use super::algebra::*;
use super::ray::*;

use glam::DVec3;

#[derive(Debug, Default, Copy, Clone)]
pub struct HitRecord {
    pub p : Point3,
    pub normal : DVec3,
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

/// What any object being rendered needs to support to validate ray intersections.
pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min:f64, t_max:f64, rec : &mut HitRecord) -> bool;
}
