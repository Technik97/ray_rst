use crate::core::vec3::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Lambertian {
    pub albedo: Vec3
}   

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}