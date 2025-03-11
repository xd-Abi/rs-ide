use crate::logging::debug;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use tracing::info;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub window: WindowConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            window: WindowConfig::default()
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            width: 800,
            height: 600,
            x: 100,
            y: 100,
        }
    }
}

pub fn load() -> Result<Config, Box<dyn Error>> {
    debug!("Attempting to load configuration from config.toml");

    let toml_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&toml_str)?;
    Ok(config)
}

pub fn save(config: &Config) -> Result<(), Box<dyn Error>> {
    debug!("Saving configuration to config.toml");

    let toml_str = toml::to_string(config)?;
    let mut file = File::create("config.toml")?;
    file.write_all(toml_str.as_bytes())?;

    info!("Configuration successfully saved");

    Ok(())
}
