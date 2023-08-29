use crate::materials::material::Material;

use super::vec3::Vec3;
use super::ray::Ray;

#[derive(Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Material
}

pub type List = Vec<Box<dyn Hittable>>;

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> ;
}

pub struct HittableList {
    list: List,
}

impl HittableList {
    pub fn new(list: List) -> Self {
        Self { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        let mut closest_so_far = tmax;

         for object in &self.list {
            if let Some(rec) =  object.hit(ray, tmin, closest_so_far) {
                closest_so_far = rec.t;
                record = Some(rec);
            }
        }

        record
    }
}