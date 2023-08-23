use super::ray::Ray;
use super::vec3::Vec3;

pub fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin() - *center;
    let a = ray.direction().length_squared();
    let half_b = Vec3::dot_product(&oc, &ray.direction);
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b -  a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
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