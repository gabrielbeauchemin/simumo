use crate::commons::Percentage;
use crate::ressources::lane_graph::EdgeId;
use crate::ressources::lane_graph::NodeId;
use serde::ser::Serialize;
use serde::ser::SerializeStruct;
use serde::ser::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use simumo_derive::simucomponent_base;
use specs::prelude::{Component, VecStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simucomponent_base]
#[derive(Debug, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub val: (EdgeId, Percentage),
}

impl Default for Position {
    fn default() -> Self {
        Self {
            val: ((0, 0), Percentage::lower()),
        }
    }
}

//todo :: remove
impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Position, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pos = PositionDeserialzable::deserialize(deserializer)?;
        Ok(Position {
            val: ((pos.from, pos.to), Percentage::new_clamp(pos.percentage)),
        })
    }
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ((from, to), percentage) = self.val;
        let mut strct = serializer.serialize_struct("position", 3)?;
        strct.serialize_field("from", &from)?;
        strct.serialize_field("to", &to)?;
        strct.serialize_field("percentage", &percentage.value())?;
        strct.end()
    }
}

#[derive(Deserialize)]
struct PositionDeserialzable {
    pub from: NodeId,
    pub to: NodeId,
    pub percentage: f64,
}
