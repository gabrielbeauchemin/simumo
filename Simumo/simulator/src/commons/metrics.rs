use crate::components::constant::Identifier;
use dim::si::{Kilogram, Meter, MeterPerSecond, MeterPerSecond2, Second};
use serde::de::Deserialize;
use serde::Deserializer;

//todo :: the deserialize should probably be removed in the future

pub type Fdim = f64;
pub type Idim = i64;
pub type Id = String;

// Function to open file instead of serializing it
pub fn fdeserialize<'de, D>(deserializer: D) -> Result<Fdim, D::Error>
where
    D: Deserializer<'de>,
{
    Fdim::deserialize(deserializer)
}

pub fn identifier_deserialize<'de, D>(deserializer: D) -> Result<Identifier, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Identifier(Id::deserialize(deserializer)?))
}

pub fn ideserialize<'de, D>(deserializer: D) -> Result<Idim, D::Error>
where
    D: Deserializer<'de>,
{
    Idim::deserialize(deserializer)
}

//todo :: reconsider how we could implement those deserialize
// in a more generic way

// Metrics Deserialiszation
pub fn second_deserialize<'de, D>(deserializer: D) -> Result<Second<Fdim>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Second::new(Fdim::deserialize(deserializer)?))
}

pub fn meter_deserialize<'de, D>(deserializer: D) -> Result<Meter<Fdim>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Meter::new(Fdim::deserialize(deserializer)?))
}

pub fn kilogram_deserialize<'de, D>(deserializer: D) -> Result<Kilogram<Fdim>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Kilogram::new(Fdim::deserialize(deserializer)?))
}

pub fn meterpersecond_deserialize<'de, D>(deserializer: D) -> Result<MeterPerSecond<Fdim>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(MeterPerSecond::new(Fdim::deserialize(deserializer)?))
}

pub fn meterpersecond2_deserialize<'de, D>(
    deserializer: D,
) -> Result<MeterPerSecond2<Fdim>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(MeterPerSecond2::new(Fdim::deserialize(deserializer)?))
}

// Metrics Serialization
