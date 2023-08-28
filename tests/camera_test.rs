#[cfg(test)]
mod test {
    use ray_tracer::setup::camera::Camera;

    #[test]
    fn test_origin() {
        let origin = Camera::origin();

        assert_eq!(origin.x(), 0.0);
        assert_eq!(origin.y(), 0.0);
        assert_eq!(origin.z(), 0.0);
    }

    #[test]
    fn test_viewport_height() {
        let viewport_height: f32 = Camera::viewport_height();

        assert_eq!(viewport_height, 2.0);
    }

    #[test]
    fn test_viewport_width() {
        let viewport_width: f32 = Camera::viewport_width();

        assert_eq!(viewport_width, 3.5555556);
    }

    #[test]
    fn test_horizontal() {
        let horizontal = Camera::horizontal();

        assert_eq!(horizontal.x(), 3.5555556);
        assert_eq!(horizontal.y(), 0.0);
        assert_eq!(horizontal.z(), 0.0);
    }

    #[test]
    fn test_vertical() {
        let vertical = Camera::vertical();

        assert_eq!(vertical.x(), 0.0);
        assert_eq!(vertical.y(), 2.0);
        assert_eq!(vertical.z(), 0.0);
    }

    #[test]
    fn test_focal_length() {
        let focal_length = Camera::focal_length();

        assert_eq!(focal_length, 1.0);
    }

    #[test]
    fn test_lower_left_corner() {
        let llc = Camera::lower_left_corner();

        assert_eq!(llc.x(), -1.7777778);
        assert_eq!(llc.y(), -1.0);
        assert_eq!(llc.z(), -1.0);
    }
}