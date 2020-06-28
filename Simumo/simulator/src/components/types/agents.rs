use crate::commons::LogDataEntry;
use crate::commons::Percentage;
use crate::ressources::lane_graph::EdgeId;
use crate::ressources::lane_graph::NodeId;
use serde::ser::Serialize;
use serde::ser::SerializeSeq;
use serde::ser::Serializer;
use simumo_derive::{simucomponent_base, simucomponent_data, SimumoSerialize};
use specs::prelude::{Component, VecStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct AcceleratingAgent {
    pub is_decelerating: bool,
}

/*#[derive(Serialize, Deserialize, Debug)]
pub struct ImmobileAgent {
    pub id: Identifier,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstantSpeedAgent {
    pub id: Identifier,
    pub position: Position,
    pub speed: Speed,
}*/

#[simucomponent_base]
#[derive(Debug)]
#[storage(VecStorage)]
pub struct Destination {
    pub val: (EdgeId, Percentage),
}

impl Default for Destination {
    fn default() -> Self {
        Self {
            val: ((0, 0), Percentage::upper()),
        }
    }
}

// I could not use an iterator here as it doesnt have constant-size at
// compile-time. path and currentIndex are used to do some kind of a cheap
// iterator
#[simucomponent_base]
#[derive(Debug)]
#[storage(VecStorage)]
pub struct Itinerary {
    pub path: Vec<NodeId>,
    pub current_index: usize,
}

impl Itinerary {
    pub fn new(path: Vec<NodeId>) -> Self {
        Self {
            path,
            current_index: 0,
        }
    }

    pub fn next(&mut self) -> Option<EdgeId> {
        let num_nodes = self.path.len();
        if self.current_index + 1 < num_nodes {
            let from = self.path[self.current_index];
            let to = self.path[self.current_index + 1];
            self.current_index += 1;
            return Some((from, to));
        }
        None
    }
}
