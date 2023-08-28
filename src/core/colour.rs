use rand::Rng;

use super::hittable::{HittableList, Hittable};
use super::ray::Ray;
use super::vec3::Vec3;

pub fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin() - *center;
    let a = ray.direction().length_squared();
    let half_b = Vec3::dot_product(&oc, &ray.direction);
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b -  a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

pub fn colour(ray: &Ray, world: &HittableList) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.0, std::f32::MAX) {
        let target = rec.p + rec.normal + random_in_unit_sphere();

        return colour(&Ray::new(rec.p, target - rec.p), &world) * 0.5;
    } else {
        let unit_direction = Vec3::unit_vector(&ray.direction());

        // let t =  hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
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