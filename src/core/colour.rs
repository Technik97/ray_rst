use rand::Rng;

use crate::materials::material::Scatterable;

use super::hittable::{HittableList, Hittable};
use super::ray::Ray;
use super::vec3::Vec3;

pub fn colour(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, std::f32::MAX) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attenuation = Vec3::default();

        if depth < 50 && rec.material.scatter(ray, &
            rec, &mut attenuation, &mut scattered) {
            return attenuation * colour(&scattered, &world,  depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = Vec3::unit_vector(&ray.direction());

        let t = (unit_direction.y() + 1.0) * 0.5;

        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    let mut rng = rand::thread_rng();

    loop {
        p = (Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0) - Vec3::new(1.0, 1.0, 1.0);

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}