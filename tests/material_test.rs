#[cfg(test)]

mod test {
    use ray_tracer::{materials::material::{self, refract}, core::vec3::Vec3};

    #[test]
    fn test_refraction() {
        let uv = Vec3::new(1.0, 1.0, 1.0);
        let n = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = 1.0;

        let res = Vec3::new(-5.3166246, -5.3166246, -5.3166246);
        let actual = refract(&uv, &n, etai_over_etat);

        assert_eq!(actual.x(), res.x());
        assert_eq!(actual.y(), res.y());
        assert_eq!(actual.z(), res.z());
    }
}
