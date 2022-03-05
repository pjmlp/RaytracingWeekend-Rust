use glam::DVec3;

// Type aliases for vec3
pub type Point3 = DVec3;   // 3D point
pub type Color = DVec3;    // RGB color

// Utility functions

pub fn unit_vector(v : DVec3) -> DVec3 {
    v / v.length()
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

pub fn random_in_unit_disk() -> DVec3 {
    // Returns a random vector in [0,1).
    loop {
        let p = DVec3::new(random_double_with(-1.0, 1.0), random_double_with(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }

        return p;
    }    
}

pub fn random_unit_vector() -> DVec3 {
    random_in_unit_sphere().normalize()
}

#[allow(dead_code)]
pub fn random_in_hemisphere(normal : DVec3) -> DVec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 { // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn near_zero(vect : &DVec3) -> bool {
    let s = 1e-8;
    (vect.x.abs() < s) && (vect.y.abs() < s) && (vect.z.abs() < s)
}

pub fn reflect (v : DVec3, n: DVec3) -> DVec3 {
    return v - 2.0 * v.dot(n) * n;
}

pub fn refract (uv : DVec3, n : DVec3, etai_over_etat:f64) -> DVec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp =  etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}