use crate::ressources::lane_graph::EdgeId;
use crate::ressources::lane_graph::EntityId;
use crate::ressources::lane_graph::LaneData;
use std::collections::HashMap;

/// Access Entry that allows to modify the LaneMap while keeping its integrity
///
/// # Fields
///
/// * `lane` :  mut ref of the currently selected lane
/// * `lane_location` : location of the lane in the graph
/// * `entity_locations` : mut ref of the mapping of all entity locations
///
/// those reference will of course be released when releasing LaneEntry
///
pub struct LaneEntry<'a, 'b> {
    lane: &'b mut LaneData,
    entity_locations: &'a mut HashMap<EntityId, EdgeId>,
}

impl<'a, 'b> LaneEntry<'a, 'b> {
    pub fn new(
        lane: &'b mut LaneData,
        entity_locations: &'a mut HashMap<EntityId, EdgeId>,
    ) -> Self {
        Self {
            lane,
            entity_locations,
        }
    }

    pub fn lane(&self) -> &LaneData {
        self.lane
    }

    pub fn push_back(&mut self, entity: EntityId) {
        self.entity_locations.insert(entity, self.lane.location());
        self.lane.push_back(entity);
    }

    pub fn pop_front(&mut self) -> EntityId {
        let entity = self.lane.pop_front();
        self.entity_locations.remove(&entity);
        entity
    }

    pub fn pop_if_front(&mut self, entity: EntityId) -> Option<EntityId> {
        let _ = self.lane.pop_if_front(entity)?;
        self.entity_locations.remove(&entity);
        Some(entity)
    }
}
