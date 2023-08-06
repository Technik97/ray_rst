use super::vec3::Vec3;
use super::ray::Ray;
use super::hittable::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool  {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot_product(&ray.direction, &ray.direction);
        let half_b = Vec3::dot_product(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
    
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < tmin || tmax < root {
            root = (-half_b + sqrtd) / a;
            if root < tmin || tmax < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}