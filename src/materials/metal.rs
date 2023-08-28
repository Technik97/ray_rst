use crate::core::{vec3::Vec3, ray::Ray, hittable::HitRecord, colour::random_in_unit_sphere};

use super::material::{Scatterable, Material, reflect};

#[derive(Debug, Clone, Copy)]
pub struct Metal { 
    pub albedo: Vec3,
    pub fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        
        let reflected = reflect(&ray_in.direction.unit_vector(), &hit_record.normal);
        *scattered = Ray::new(hit_record.p, reflected + random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;

        Vec3::dot_product(&scattered.direction, &hit_record.normal) > 0.0
    }
}