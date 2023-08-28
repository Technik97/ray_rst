use crate::configuration;

#[derive(Debug)]
pub struct Image {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub image_height: i32,
}

impl Image {
    pub fn aspect_ratio() -> f32 {
        let config = configuration::get_configuration().unwrap();
        let aspect_width = config.get("aspect_width").unwrap().parse::<f32>().unwrap();
        let aspect_height = config.get("aspect_height").unwrap().parse::<f32>().unwrap();

        aspect_width / aspect_height
    } 

    pub fn image_width() -> i32 {
        let config = configuration::get_configuration().unwrap();
        let image_width = config.get("image_width").unwrap().parse::<i32>().unwrap();

        image_width
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