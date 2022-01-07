use super::algebra::*;

use glam::DVec3;

#[derive(Debug)]
pub struct Ray {
    dir : DVec3,
    orig : Point3
}

impl Default for Ray {
    fn default() -> Ray {
        Ray {
            dir: DVec3::new(0.0, 0.0,0.0),
            orig: DVec3::new(0.0, 0.0,0.0)
        }
    }
}



impl Ray {
    pub fn new(origin : DVec3, direction: Point3) -> Ray {
        Ray {
            dir: direction,
            orig: origin
        }
    }

    pub fn direction(&self) -> DVec3 {
        self.dir
    }

    pub fn origin(&self) -> DVec3 {
        self.orig
    }

    pub fn at(&self, t: f64) -> DVec3 {
        return self.orig + t*self.dir;
    }
}
