use super::algebra::*;
use super::hitable::*;
use super::ray::*;

pub struct Lambertian {
    pub albedo : Color
}

impl Lambertian {
    pub fn new(a : Color) -> Self {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }
        

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    pub albedo : Color
}

impl Metal {
    pub fn new(a : Color) -> Self {
        Metal { albedo: a }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);

        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        scattered.direction().dot(rec.normal) > 0.0
    }
}