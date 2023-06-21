use std::fs::File;

use image::{DynamicImage, ColorType, ImageEncoder, GenericImageView, ImageBuffer};
use image::io::Reader as ImageReader;

use image::codecs::png::PngEncoder;

use palette::{Srgb};
use palette::rgb::Rgb;

use crate::core::ray::Ray;
use crate::core::vec3::Vec3;

pub fn ray_color(ray: &Ray) -> Rgb {
   let unit_direction: Vec3 = Vec3::unit_vector(&ray.direction());
   let t: f32 = 0.5 * (unit_direction.y() + 1.0);

   Srgb::new(
    (1.0 - t) * 1.0 + t * 0.5,
    (1.0 - t) * 1.0 + t * 0.7,
    (1.0 - t) * 1.0 + t * 1.0,
   )
}

pub fn img_manupilation(filename: &str) -> DynamicImage {
    let img = ImageReader::open(filename).unwrap().decode();

    img.unwrap()
}

pub fn img_write() -> Result<(), std::io::Error> {
    let output = File::create("output.png")?;
    let encoder = PngEncoder::new(output);

    let pixels = vec![0; 1280 * 720 * 3];

    encoder.write_image(&pixels, 1280, 720, ColorType::Rgb8).unwrap();

    Ok(())
}

