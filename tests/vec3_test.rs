#[cfg(test)]
mod test {
    use ray_tracer::core::vec3::Vec3;

    #[test]
    fn test_new() {
        let point: Vec3 = Vec3::new(5.0, 4.0, 3.0);

        assert_eq!(point.x(), 5.0);
        assert_eq!(point.y(), 4.0);
        assert_eq!(point.z(), 3.0);
    }

    #[test]
    fn test_dot() {
        let u: Vec3 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v: Vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

        let dot_product: f32 = Vec3::dot_product(&u, &v);

        assert_eq!(dot_product, 32.0);
    }

    #[test]
    fn test_cross() {
        let u: Vec3 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v: Vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

        let cross_product = Vec3::cross_product(&u, &v);

        assert_eq!(cross_product.x(), -3.0);
        assert_eq!(cross_product.y(), 6.0);
        assert_eq!(cross_product.z(), -3.0);
    }

    #[test]
    fn test_length(){
        let r: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

        let length: f32 = r.length();

        assert_eq!(length, 1.7320508);
    }

    #[test]
    fn test_unit_vector() {
        let r: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

        let unit = r.unit_vector();

        assert_eq!(unit.x(), 0.57735026);
        assert_eq!(unit.y(), 0.57735026);
        assert_eq!(unit.z(), 0.57735026);
    }

}