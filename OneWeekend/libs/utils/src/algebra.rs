use glam::DVec3;

// Type aliases for vec3
pub type Point3 = DVec3;   // 3D point
pub type Color = DVec3;    // RGB color

// Utility functions

pub fn unit_vector(v : DVec3) -> DVec3 {
    v / v.length()
}