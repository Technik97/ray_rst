#[cfg(test)]
mod test {
    use ray_tracer::core::{vec3::Vec3, ray::Ray};

    #[test]
    fn test_new() {
        let origin: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let direction: Vec3 = Vec3::new(2.0, 2.0, 2.0);

        let r: Ray = Ray::new(origin, direction);

        assert_eq!(r.origin.x(), 1.0);
        assert_eq!(r.origin.y(), 1.0);
        assert_eq!(r.origin.z(), 1.0);

        assert_eq!(r.direction.x(), 2.0);
        assert_eq!(r.direction.y(), 2.0);
        assert_eq!(r.direction.z(), 2.0);
    }

    #[test]
    fn test_at() {
        let origin: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let direction: Vec3 = Vec3::new(2.0, 2.0, 2.0);

        let r: Ray = Ray::new(origin, direction);

        let at = r.point_at_parameter(1.0);

        assert_eq!(at.x(), 3.0);
        assert_eq!(at.y(), 3.0);
        assert_eq!(at.z(), 3.0);
    }
}