mod intersection_data;
mod lane_data;
mod lane_entry;
pub use self::intersection_data::IntersectionData;
pub use self::lane_data::LaneData;
pub use self::lane_entry::LaneEntry;

use petgraph::algo::astar;
use petgraph::graphmap::DiGraphMap;
use petgraph::graphmap::GraphMap;
use petgraph::IntoWeightedEdge;
use specs::world;
use std::collections::HashMap;

pub type NodeId = u64;
pub type EdgeId = (NodeId, NodeId);

/// The Identifier of the entities in the graph
/// it uses the entities ID of specs
pub type EntityId = world::Index;

/// A GraphMap of the map of the lane
///
/// # Fields
///
/// * `graph` - graph containing information of the lanes base on the `IntersectionId`
/// * `intersections` -  mapping of the intersection data based on their `IntersectionId`
/// * `entity_locations` - locations of the entities in the graph
///
pub struct LaneGraph {
    pub graph: DiGraphMap<NodeId, EdgeId>,
    pub intersections: HashMap<NodeId, IntersectionData>,
    pub entity_locations: HashMap<EntityId, EdgeId>,
    pub lanes: HashMap<EdgeId, LaneData>,
}

impl LaneGraph {
    pub fn new<I1, I2>(nodes: I1, edges: I2) -> Self
    where
        I1: Iterator<Item = (NodeId, IntersectionData)>,
        I2: IntoIterator,
        I2::Item: IntoWeightedEdge<LaneData, NodeId = NodeId>,
    {
        // Associate EdgeId to LaneData
        let mut edge_ids = Vec::new();
        let mut lanes = HashMap::new();
        for edge in edges {
            let (node_from, node_to, lane_data) = edge.into_weighted_edge();
            let edge_id = (node_from, node_to);
            edge_ids.push(edge_id);
            lanes.insert(edge_id, lane_data);
        }
        // Return LaneGraph
        Self {
            graph: GraphMap::from_edges(edge_ids),
            intersections: nodes.collect::<HashMap<_, _>>(),
            entity_locations: HashMap::new(),
            lanes,
        }
    }

    /// Take the entity in front of the lane `from`
    /// and put it at the back of the lane `to`
    ///
    pub fn node_forward(&mut self, from: EdgeId, to: EdgeId) {
        let front_entity = { self.lane_between_mut(from).unwrap().pop_front() };
        self.lane_between_mut(to).unwrap().push_back(front_entity);
    }

    /// forward a tuple of three node
    ///
    pub fn segment_forward(&mut self, (from, middle, to): (NodeId, NodeId, NodeId)) {
        self.node_forward((from, middle), (middle, to));
    }

    /// Take the selected entity ID in the end of a lane
    /// and then move it to the front of an other lane
    ///
    pub fn entity_forward(&mut self, entity: EntityId, destination: NodeId) {
        let (begin, end) = self.entity_locations[&entity];
        self.segment_forward((begin, end, destination));
    }

    // Method to access

    pub fn intersections(&self) -> &HashMap<NodeId, IntersectionData> {
        &self.intersections
    }
    pub fn lanes(&self) -> &DiGraphMap<NodeId, EdgeId> {
        &self.graph
    }
    pub fn entity_locations(&self) -> &HashMap<EntityId, (NodeId, NodeId)> {
        &self.entity_locations
    }

    /// get a reference of the intersection
    ///
    pub fn intersection(&self, entity: NodeId) -> &IntersectionData {
        &self.intersections[&entity]
    }

    /// get a mutable reference on the intersection
    ///
    pub fn intersection_mut(&mut self, entity: NodeId) -> &mut IntersectionData {
        self.intersections.get_mut(&entity).unwrap()
    }

    /// get the lane with entity id
    ///
    pub fn lane(&self, entity: EntityId) -> &LaneData {
        let location = self.entity_locations[&entity];
        self.lane_between(location).unwrap()
    }

    /// get the lane between two nodes
    ///
    pub fn lane_between(&self, location: EdgeId) -> Option<&LaneData> {
        self.lanes.get(&location)
    }

    /// get the lane as a mutable lane based on the entityId
    ///
    pub fn lane_mut(&mut self, entity: EntityId) -> LaneEntry {
        let location = self.entity_locations[&entity];
        self.lane_between_mut(location).unwrap()
    }

    /// Get the lane as a mutable lane between two nodes
    ///
    pub fn lane_between_mut(&mut self, location: EdgeId) -> Option<LaneEntry> {
        if let Some(lane) = self.lanes.get_mut(&location) {
            return Some(LaneEntry::new(lane, &mut self.entity_locations));
        }
        None
    }

    pub fn get_edge_cost(&self, edge_id: EdgeId) -> f64 {
        if let Some(lane) = self.lane_between(edge_id) {
            return lane.get_cost_index();
        }
        std::f64::INFINITY
    }

    pub fn get_estimate_cost_from_node(&self, node_id: NodeId, goal: NodeId) -> f64 {
        let node_position = self.intersection(node_id).position();
        let goal_position = self.intersection(goal).position();
        let diff_x = goal_position.0 - node_position.0;
        let diff_y = goal_position.1 - node_position.1;
        diff_x.hypot(diff_y)
    }

    pub fn get_optimal_path_between_nodes(
        &self,
        start_node: NodeId,
        end_node: NodeId,
    ) -> Option<(f64, Vec<NodeId>)> {
        astar(
            self.lanes(),
            start_node,
            |finish| finish == end_node,
            |e| self.get_edge_cost((e.0, e.1)),
            |n| self.get_estimate_cost_from_node(n, end_node),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commons::Curve;

    /// get a map in a triangle :
    /// it uses stub position in the nodes
    ///
    ///   1
    ///    \
    ///     3 --> 4
    ///    /
    ///   2
    ///
    fn lane_map_triangle() -> LaneGraph {
        let node = IntersectionData::new(10.0, 10.0);

        use crate::commons::Point2D;
        LaneGraph::new(
            [
                (1u64, node.clone()),
                (2u64, node.clone()),
                (3u64, node.clone()),
                (4u64, node.clone()),
            ]
            .to_vec()
            .into_iter(),
            &[
                (
                    1,
                    3,
                    LaneData::new(
                        (1, 3),
                        None,
                        None,
                        Curve::new(vec![Point2D { x: 0.0, y: 0.0 }, Point2D { x: 0.0, y: 0.0 }]),
                    ),
                ),
                (
                    2,
                    3,
                    LaneData::new(
                        (2, 3),
                        None,
                        None,
                        Curve::new(vec![Point2D { x: 0.0, y: 0.0 }, Point2D { x: 0.0, y: 0.0 }]),
                    ),
                ),
                (
                    3,
                    4,
                    LaneData::new(
                        (3, 4),
                        None,
                        None,
                        Curve::new(vec![Point2D { x: 0.0, y: 0.0 }, Point2D { x: 0.0, y: 0.0 }]),
                    ),
                ),
            ],
        )
    }

    #[test]
    fn push_valid() {
        let mut graph = lane_map_triangle();
        graph.lane_between_mut((1, 3)).unwrap().push_back(1);
        graph.lane_between_mut((1, 3)).unwrap().push_back(2);
        let mut lane = graph.lane_between_mut((1, 3)).unwrap();
        assert_eq!(lane.lane().queue().len(), 2);
        assert_eq!(lane.pop_front(), 1);
        assert_eq!(lane.pop_front(), 2);
        assert!(lane.lane().queue().is_empty());
    }

    #[test]
    fn node_forward_gives_2314() {
        let mut graph = lane_map_triangle();
        graph.lane_between_mut((1, 3)).unwrap().push_back(1);
        graph.lane_between_mut((1, 3)).unwrap().push_back(2);
        graph.lane_between_mut((2, 3)).unwrap().push_back(3);
        graph.lane_between_mut((3, 4)).unwrap().push_back(4);

        graph.node_forward((1, 3), (3, 4));
        graph.node_forward((2, 3), (3, 4));
        graph.node_forward((1, 3), (3, 4));

        let lane = graph.lane_between((3, 4)).unwrap();
        assert_eq!(lane.queue().get(0).unwrap(), &4);
        assert_eq!(lane.queue().get(1).unwrap(), &1);
        assert_eq!(lane.queue().get(2).unwrap(), &3);
        assert_eq!(lane.queue().get(3).unwrap(), &2);

        assert_eq!(lane.queue().len(), 4);
        assert!(graph.lane_between((1, 3)).unwrap().queue().is_empty());
        assert!(graph.lane_between((2, 3)).unwrap().queue().is_empty());
    }
}
