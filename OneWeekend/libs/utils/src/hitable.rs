use super::algebra::*;
use super::ray::*;

use glam::DVec3;

#[derive(Debug)]
pub struct HitRecord {
    pub p : Point3,
    pub normal : DVec3,
    pub t : f64,
    pub front_face: bool
}

/// What any object being rendered needs to support to validate ray intersections.
pub trait Hitable {
    fn hit(self, ray: &Ray, t_min:f64, t_max:f64, rec : &mut HitRecord) -> bool;
}
