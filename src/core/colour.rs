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
        return Vec3::new(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0) * 0.5;
    } else {
        // let direction: Vec3 = ray.direction();
        let t =  hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray);

        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}