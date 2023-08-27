use super::vec3::Vec3;
use super::ray::Ray;

#[derive(Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool
}

pub type List = Vec<Box<dyn Hittable>>;

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool ;
    // fn set_face_normal(self, ray: &Ray, outward_noraml: &Vec3);
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
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec:  &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = tmax;

         for object in &self.list {
            if object.hit(ray, tmin, closest_so_far, &tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                rec = &mut tmp_rec;
            }
        }

        hit_anything
    }
}