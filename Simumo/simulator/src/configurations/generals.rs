/*! Define general configuration. */

use crate::commons::metrics::second_deserialize;
use crate::commons::metrics::Fdim;
use crate::configurations::debugger::VisualDebugger;
use dim::si::{Second, MIN};
use rts_logger::data_writer::{DataWrite, NdJsonWriter};
use rts_logger::LogWriterManager;
use rts_logger::LoggerConfiguration;
use std::fs;

#[derive(Deserialize)]
pub struct GeneralConfigurations {
    #[serde(deserialize_with = "second_deserialize")]
    ///Represent the step ins second between each tick of clock.
    pub clock_dt: Second<Fdim>,
    pub end_time: EndTime,
    pub debugger: VisualDebugger,
    pub logging: Option<LoggingConfiguration>,
    pub seed: String,
    pub random_speed: Option<bool>
}

///
///
#[derive(Deserialize)]
pub struct LoggingConfiguration {
    pub path: String,
    pub filenames: Vec<String>,
}
impl LoggingConfiguration {
    pub fn get_manager(&self) -> LogWriterManager {
        let _ = fs::remove_dir_all(&self.path);
        let _ = fs::create_dir(&self.path);
        let loggers: Vec<LoggerConfiguration> = self
            .filenames
            .iter()
            .map(|name| {
                let file_path = format!("{}/{}", self.path, name.clone());
                LoggerConfiguration {
                    name: name.clone(),
                    data_writer: Box::new(NdJsonWriter::open(&file_path)),
                }
            })
            .collect();
        LogWriterManager::from_loggers(loggers.into_iter()).unwrap()
    }
}

#[derive(Clone, Deserialize)]
pub struct EndTime {
    #[serde(deserialize_with = "second_deserialize")]
    pub val: Second<Fdim>,
}

impl Default for EndTime {
    fn default() -> Self {
        Self { val: MIN }
    }
}
