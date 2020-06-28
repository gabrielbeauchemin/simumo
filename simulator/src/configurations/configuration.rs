/*! Contain all configurations */

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use super::generals::GeneralConfigurations;
use super::map::Map;
use super::systems::SystemsConfiguration;
use crate::entities::entity_type::EntityType;

/// Represent the root level configuration.
// Todo: Can't handle empty field in serialization.
#[derive(Deserialize)]
pub struct Configuration {
    pub generals: GeneralConfigurations,
    pub map: Map,
    pub systems: SystemsConfiguration,
    pub entities: Option<Vec<EntityType>>,
}

impl Configuration {
    ///import config from json file.
    pub fn from_json(args_path: &str) -> Result<Self, Box<Error>> {
        let config_path = Path::new(&args_path);
        let file = File::open(config_path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    ///import config from yaml file.
    pub fn from_yaml(args_path: &str) -> Result<Self, Box<Error>> {
        let config_path = Path::new(&args_path);
        let file = File::open(config_path)?;
        let reader = BufReader::new(file);
        let config = serde_yaml::from_reader(reader)?;
        Ok(config)
    }
}
