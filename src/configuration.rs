use config::{ Config, File };

use crate::core::vec3::Vec3;

#[derive(serde::Deserialize)]
pub struct SceneSettings {
    pub image: ImageSettings,
    pub camera: CameraSettings
}

#[derive(serde::Deserialize)]
pub struct ImageSettings {
    pub width: u32,
    pub heigth: u32,
    pub aspect_width: f32,
    pub aspect_heigth: f32,
}

#[derive(serde::Deserialize)]
pub struct CameraSettings {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

pub fn get_configuration() ->  Result<SceneSettings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(File::new("configuration.yaml", config::FileFormat::Yaml))
        .build()?;

    settings.try_deserialize::<SceneSettings>()
}