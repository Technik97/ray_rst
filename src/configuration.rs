use std::collections::HashMap;
use config::{Config, File};

pub fn get_configuration() -> Result<HashMap<String, String>, config::ConfigError> {
    let settings: Config = Config::builder()
        .add_source(File::new("configuration.toml", config::FileFormat::Toml))
        .build()?;

    settings.try_deserialize::<HashMap<String, String>>()
}