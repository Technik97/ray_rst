use crate::materials::material::Material;

use super::vec3::Vec3;
use super::ray::Ray;
use super::hittable::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>  {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot_product(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
    
        let discriminant = half_b * half_b - a * c;

        if discriminant >= 0.0 {
            let temp = (- half_b - discriminant.sqrt()) / a;
            let normal = (ray.point_at_parameter(temp) - self.center) / self.radius;
            let front_face = Vec3::dot_product(&ray.direction, &normal) < 0.0;

            if temp < tmax && temp > tmin {
                return Some(HitRecord { 
                    t: temp, 
                    p: ray.point_at_parameter(temp), 
                    normal: if front_face { normal } else { -normal }, 
                    front_face,
                    material: self.material
                });
            }
        }
        None
    }
}