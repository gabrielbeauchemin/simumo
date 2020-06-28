use serde::ser::Serialize;
use serde::ser::SerializeStruct;
use serde::ser::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use simumo_derive::simucomponent_base;
use specs::prelude::{Component, VecStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

use crate::ressources::lane_graph::NodeId;

#[simucomponent_base]
#[derive(Debug, Clone)]
#[storage(VecStorage)]
pub struct StaticPosition {
    pub val: NodeId,
}

impl Default for StaticPosition {
    fn default() -> Self {
        Self { val: 0 }
    }
}

//todo :: remove
impl<'de> Deserialize<'de> for StaticPosition {
    fn deserialize<D>(deserializer: D) -> Result<StaticPosition, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pos = StaticPositionDeserialzable::deserialize(deserializer)?;
        Ok(StaticPosition { val: pos.node })
    }
}

impl Serialize for StaticPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strct = serializer.serialize_struct("StaticPosition", 1)?;
        strct.serialize_field("node", &self.val)?;
        strct.end()
    }
}

#[derive(Deserialize)]
struct StaticPositionDeserialzable {
    pub node: NodeId,
}
