use crate::commons::metrics::Fdim;
use crate::commons::Curve;
use crate::ressources::lane_graph::EdgeId;
use crate::ressources::lane_graph::EntityId;
use dim::si::{Meter, MeterPerSecond};
use std::collections::VecDeque;

/// Contains all the information of a lane in the map
///
/// # Fields
///
/// * `entity_queue` - ordered queue giving the order of the contained elements
/// * `width` - width of the lane
/// * `max_speed` - max speed of the lane
/// * `curve` - curve of the lane
///
/// note :: `width`,`max_speed` and `curve`are options because we
///     are not guaranteed yet to have it for every lane
///
/// Contains all the information of a lane in the map
///
/// # Fields
///
/// * `entity_queue` - ordered queue giving the order of the contained elements
/// * `width` - width of the lane
/// * `max_speed` - max speed of the lane
/// * `curve` - curve of the lane
///
/// note :: `width`,`max_speed` and `curve`are options because we
///     are not garrenteed yet to have it for everylane
#[derive(Clone, Debug)]
pub struct LaneData {
    location: EdgeId,
    entity_queue: VecDeque<EntityId>,
    //todo :: consider if all the specific data  (width,max_speed,etc)
    // should be wrapped in a generic this way we could  abstract street info
    // from the graph w
    width: Option<Meter<Fdim>>,
    max_speed: Option<MeterPerSecond<Fdim>>,
    curve: Curve,
}

impl LaneData {
    pub fn new(
        location: EdgeId,
        width: Option<Meter<Fdim>>,
        max_speed: Option<MeterPerSecond<Fdim>>,
        curve: Curve,
    ) -> Self {
        Self {
            location,
            entity_queue: VecDeque::new(),
            width,
            max_speed,
            curve,
        }
    }

    pub fn location(&self) -> EdgeId {
        self.location
    }

    pub fn curve(&self) -> &Curve {
        &self.curve
    }

    pub fn max_speed(&self) -> Option<MeterPerSecond<Fdim>> {
        self.max_speed
    }

    /// get a reference of the queue
    ///
    pub fn queue(&self) -> &VecDeque<EntityId> {
        &self.entity_queue
    }

    /// Insert a entity at the beginning of the lane
    ///
    /// note :: we use the back of de entity queue because
    ///         it makes more sense in our context
    pub fn push_back(&mut self, entity: EntityId) {
        self.entity_queue.push_back(entity);
    }

    /// pop an entity at the end of the lane
    ///
    ///
    pub fn pop_front(&mut self) -> EntityId {
        self.entity_queue.pop_front().unwrap()
    }

    /// remove if the entity is in front of the queue
    ///
    /// todo :: result instead of option?
    pub fn pop_if_front(&mut self, entity: EntityId) -> Option<EntityId> {
        let front_entity = self.entity_queue.front()?;
        if *front_entity != entity {
            None
        } else {
            self.entity_queue.pop_front()
        }
    }

    /// give the entity which is in front of an other entity
    ///
    pub fn in_front_of(&self, entity: EntityId) -> EntityId {
        let pos = self.entity_queue.iter().position(|x| x == &entity).unwrap();
        self.entity_queue[pos + 1]
    }

    // An index that is used for the A* algorithm in order to estimate the
    // cost of a lane.
    // The higher the index, the higher the cost of the lane
    pub fn get_cost_index(&self) -> f64 {
        let _nb_entities: usize = self.entity_queue.len();
        self.curve.length().value_unsafe
    }
}
