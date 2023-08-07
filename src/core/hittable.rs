use super::vec3::Vec3;
use super::ray::Ray;

#[derive(Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool ;
    fn set_face_normal(self, ray: &Ray, outward_noraml: &Vec3);
}