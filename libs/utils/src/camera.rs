use super::algebra::*;
use super::ray::*;

use glam::DVec3;

pub struct Camera {
    origin : Point3,
    lower_left_corner : Point3,
    horizontal : DVec3,
    vertical : DVec3,
}

impl Camera {
    pub fn new(
        lookfrom : Point3,
        lookat : Point3,
        vup : DVec3,
        vfov : f64, // vertical field-of-view in degrees
        aspect_ratio : f64) -> Self {

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        
        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Camera {origin: origin, horizontal:horizontal, vertical:vertical, lower_left_corner:lower_left_corner}
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
    }
}
