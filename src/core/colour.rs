use super::ray::Ray;
use super::vec3::Vec3;

pub fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - *center;
    let a = Vec3::dot_product(&ray.direction, &ray.direction);
    let b = 2.0 * Vec3::dot_product(&oc, &ray.direction);
    let c = Vec3::dot_product(&oc, &oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

pub fn colour(ray: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let direction: Vec3 = ray.direction();
    let v = direction.unit_vector();
    let unit_direction = v.unit_vector();

    let t: f32 = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}