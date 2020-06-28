use crate::ressources::lane_graph::EntityId;

///  Contains all the information of an intersection in the map
///
///  # Fields
///
/// * `position` - position in longitude latitude
/// * `contained_entity` - Index referencing to the contained entity
///
#[derive(Clone, Debug)]
pub struct IntersectionData {
    position: (f64, f64),
    contained_entity: Option<EntityId>,
}

impl IntersectionData {
    pub fn new(lon: f64, lat: f64) -> Self {
        Self {
            position: (lon, lat),
            contained_entity: None,
        }
    }

    pub fn position(&self) -> (f64, f64) {
        self.position
    }
}
