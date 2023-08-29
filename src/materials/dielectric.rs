use crate::core::{ray::Ray, hittable::HitRecord, vec3::Vec3};

use super::material::{Scatterable, refract, reflect};

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub ir: f32
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(&ray_in.direction());
        let cos_theta = Vec3::dot_product( &(-unit_direction), &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut direction = Vec3::default();

        if cannot_refract {
            direction = reflect(&unit_direction, &hit_record.normal);
        } else {
            direction = refract(&unit_direction, &hit_record.normal, refraction_ratio);
        }

        *scattered = Ray::new(hit_record.p, direction);

        true
    }
}