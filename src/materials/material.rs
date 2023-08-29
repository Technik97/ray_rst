use crate::core::{ray::Ray, hittable::HitRecord, vec3::Vec3};

use super::{lambertian::Lambertian, metal::Metal, dielectric::{Dielectric, self}};


#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric)
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Lambertian::default())
    }
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * (2.0 * Vec3::dot_product(v, n))
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = Vec3::dot_product(&(-*uv), n).min(1.0);
    
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * (-1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt());

    r_out_perp + r_out_parallel
}

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(lambertian) => {
                lambertian.scatter(ray_in, hit_record, attenuation, scattered)
            },
            Material::Metal(metal) => {
                metal.scatter(ray_in, hit_record, attenuation, scattered)
            },
            Material::Dielectric(dielectric) => {
                dielectric.scatter(ray_in, hit_record, attenuation, scattered)
            },
        }
    }
}