use crate::core::{ray::Ray, hittable::HitRecord, vec3::Vec3};

use super::{lambertian::Lambertian, metal::Metal, dielectric::Dielectric};


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

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(lambertian) => {
                lambertian.scatter(ray_in, hit_record, attenuation, scattered)
            },
            Material::Metal(metal) => {
                metal.scatter(ray_in, hit_record, attenuation, scattered)
            },
            Material::Dielectric(_) => todo!(),
        }
    }
}