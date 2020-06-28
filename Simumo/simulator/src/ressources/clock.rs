use crate::commons::metrics::Fdim;

use dim::si::{Second, S};
use serde::Deserialize;
use serde::Deserializer;

pub struct Clock {
    pub dt: Second<Fdim>,
    tick: i32,
}

impl<'de> Deserialize<'de> for Clock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Clock::new(Second::new(Fdim::deserialize(deserializer)?)))
    }
}

impl Clock {
    pub fn new(dt: Second<Fdim>) -> Clock {
        Clock { dt, tick: 0 }
    }
    pub fn update(&mut self) {
        self.tick += 1;
    }
    pub fn get_time(&self) -> Second<Fdim> {
        self.dt * f64::from(self.tick)
    }
    pub fn get_dt(&self) -> Second<Fdim> {
        self.dt
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            dt: 1.0 * S,
            tick: 0,
        }
    }
}
