use std::rc::Rc;
use glam::DVec3;
use super::algebra::*;
use super::hitable::*;
use super::material::*;
use super::sphere::*;

pub fn random_scene() -> HitableList {
    let mut world = HitableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(DVec3::new( 0.0,-1000.0, 0.0), 1000.0, Some(ground_material))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new((a as f64) + 0.9 * random_double(), 0.2, (b as f64) + 0.9*random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material : Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_dvec3() * random_dvec3();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_dvec3_with(0.5, 1.0);
                    let fuzz = random_double_with(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));

                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                }

                world.add(Rc::new(Sphere::new(center, 0.2, Some(sphere_material))));

            } 
        }
    }


    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(DVec3::new( 0.0, 1.0, 0.0), 1.0, Some(material1))));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(DVec3::new( -4.0, 1.0, 0.0), 1.0, Some(material2))));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(DVec3::new( 4.0, 1.0, 0.0), 1.0, Some(material3))));

    world
}