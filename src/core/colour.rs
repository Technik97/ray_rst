use super::ray::Ray;
use super::vec3::Vec3;

pub fn colour(ray: &Ray) -> Vec3 {
    let direction: Vec3 = ray.direction();
    let v = direction.unit_vector();
    let unit_direction = v.unit_vector();

    let t: f32 = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}