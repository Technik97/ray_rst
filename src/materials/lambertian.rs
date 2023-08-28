use crate::core::{vec3::Vec3, ray::Ray, hittable::HitRecord, colour::random_in_unit_sphere};

use super::material::Scatterable;

#[derive(Debug, Clone, Copy, Default)]
pub struct Lambertian {
    pub albedo: Vec3
}   

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attentuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        *scattered = Ray::new(hit_record.p, target - hit_record.p);
        *attentuation = self.albedo;

        true
    }
}