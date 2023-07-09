#[derive(Debug)]
pub struct Image {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub image_height: i32,
}

impl Image {
    pub fn aspect_ratio() -> f32 {
        16.0 / 9.0
    } 

    pub fn image_width() -> i32 {
        640
    }

    pub fn image_height() -> i32 {
        let img_width: i32 = Self::image_width();
        let asp_ratio: f32 = Self::aspect_ratio();

        let img_height: f32 = (img_width as f32) / asp_ratio;

        img_height as i32
    }

    pub fn new() -> Self {
        let image_width: i32 = Self::image_width();
        let aspect_ratio: f32 = Self::aspect_ratio();

        let image_height: i32 = Self::image_height();

        Image { aspect_ratio, image_width, image_height }
    }
}