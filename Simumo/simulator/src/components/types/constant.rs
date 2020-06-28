/*! Define constant components. */

use crate::commons::metrics::Fdim;
use crate::commons::metrics::{kilogram_deserialize, meter_deserialize};
use crate::commons::LogDataEntry;
use crate::systems::renderer::drawableshape::DrawableShape;

use dim::si::{Kilogram, Meter};
use serde::ser::Serialize;
use serde::ser::SerializeSeq;
use serde::ser::Serializer;
use simumo_derive::{simucomponent_data, simucomponent_tag, SimumoSerialize};
use specs::prelude::{Component, VecStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Length {
    #[simumo_metric]
    #[serde(deserialize_with = "meter_deserialize")]
    pub val: Meter<Fdim>,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Mass {
    #[simumo_metric]
    #[serde(deserialize_with = "kilogram_deserialize")]
    pub val: Kilogram<Fdim>,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Identifier(pub String);

//entity types
#[simucomponent_tag]
#[storage(VecStorage)]
pub struct CarType;

#[simucomponent_tag]
#[storage(VecStorage)]
pub struct BikeType;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Drawer {
    pub figure: DrawableShape,
}
