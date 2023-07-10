use super::{vec3::Vec3, img::Image};
use crate::configuration;

#[derive(Debug)]
pub struct Camera {
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3
}


impl Camera {
    pub fn origin() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn viewport_height() -> f32 {
        let config = configuration::get_configuration().unwrap();
        let viewport_height = config.get("viewport_height").unwrap().parse::<f32>().unwrap();

        viewport_height
    }

    pub fn viewport_width() -> f32 {
        let asp_ratio: f32 = Image::aspect_ratio();
        let vp_height: f32 = Self::viewport_height();

        asp_ratio * vp_height
    }

    pub fn horizontal() -> Vec3 {
        let vp_width: f32 = Self::viewport_width();

        Vec3 {x: vp_width , y: 0.0, z: 0.0 }
    }

    pub fn vertical() -> Vec3 {
        let vp_height: f32 = Self::viewport_height();

        Vec3 { x: 0.0, y: vp_height, z: 0.0 }
    }

    pub fn focal_length() -> f32 {
        let config = configuration::get_configuration().unwrap();
        let focal_length = config.get("focal_length").unwrap().parse::<f32>().unwrap();
        
        focal_length
    }

    pub fn lower_left_corner() -> Vec3 {
        let origin = Self::origin();
        let focal_length = Self::focal_length();
        let horizontal = Self::horizontal();
        let vertical = Self::vertical();

        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3 { x: 0.0, y: 0.0, z: focal_length }
    }

    pub fn new() -> Self {
        let viewport_height = Self::viewport_height();
        let viewport_width = Self::viewport_width();
        let origin = Self::origin();
        let focal_length = Self::focal_length();
        let horizontal = Self::horizontal();
        let vertical = Self::vertical();
        let lower_left_corner = Self::lower_left_corner();


        Self { viewport_height, viewport_width, focal_length, origin, horizontal, lower_left_corner, vertical }
    }
}