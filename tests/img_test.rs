#[cfg(test)]
mod test {
    use ray_tracer::core::img::Image;

    #[test]
    fn test_aspect_ratio() {
        let asp_ratio: f32 = Image::aspect_ratio();

        assert_eq!(asp_ratio, 1.7777778);
    }

    #[test]
    fn test_image_width() {
        let image_width: i32 = Image::image_width();

        assert_eq!(image_width, 640);
    }

    #[test]
    fn test_image_height() {
        let image_height: i32 = Image::image_height();

        assert_eq!(image_height, 360);
    }

    #[test]
    fn test_new() {
        let img = Image::new();

        assert_eq!(img.aspect_ratio, 1.7777778);
        assert_eq!(img.image_width, 640);
        assert_eq!(img.image_height, 360);
    }
}