use super::ray::Ray;
use super::vec3::Vec3;

pub fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin() - *center;
    let a = Vec3::dot_product(&ray.direction, &ray.direction);
    let b = 2.0 * Vec3::dot_product(&oc, &ray.direction);
    let c = Vec3::dot_product(&oc, &oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / 2.0 * a;
    }
}

pub fn colour(ray: &Ray) -> Vec3 {
    let t =  hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    
    if t > 0.0 {
        let v = ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0);
        let N = v.unit_vector();
        return Vec3::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) * 0.5;
    }

    let direction: Vec3 = ray.direction();
    let v = direction.unit_vector();
    let unit_direction = v.unit_vector();

    let t: f32 = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}