use crate::commons::CartesianCoord;
use crate::commons::Percentage;
use crate::commons::PolarCoord;
use crate::components::types::agents::Destination;
use crate::components::types::agents::Itinerary;
use crate::components::types::constant::CarType;
use crate::components::types::constant::Drawer;
use crate::components::types::constant::Identifier;
use crate::components::types::dynamic::Speed;
use crate::components::Position;
use crate::entities::entity_type::Instantiable;
use crate::ressources::lane_graph::LaneGraph;
use crate::ressources::lane_graph::NodeId;
use crate::systems::renderer::drawableshape::DrawableShape;
use crate::systems::renderer::drawableshape::Rectangle;
use dim::si::MPS;
use specs::prelude::{Builder, Entities, LazyUpdate, Read, World};
use specs::EntityBuilder;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Serialize, Deserialize, Debug)]
pub struct CarEntity {
    pub id: String,
    //mass : Mass,
    //length : Length,
    //angle: Angle,
    #[serde(default)]
    pub position: ((NodeId, NodeId), f64),
    #[serde(default)]
    pub destination: ((NodeId, NodeId), f64),
    #[serde(default)]
    pub speed: f64,
    #[serde(default)]
    pub acceleration: f64,
    pub path: Option<Vec<NodeId>>,
    //energy_control: EnergyControl,
    //agent_type:
}

impl CarEntity {
    pub fn new(
        start_node: NodeId,
        end_node: NodeId,
        speed: f64,
        acceleration: f64,
        lane_graph: &LaneGraph,
    ) -> Self {
        ID.fetch_add(1, Ordering::SeqCst);
        let (_cost, path) = lane_graph
            .get_optimal_path_between_nodes(start_node, end_node)
            .unwrap();
        let num_nodes = path.len();

        Self {
            id: ID.load(Ordering::SeqCst).to_string(),
            position: ((path[0], path[1]), 0.0),
            destination: ((path[num_nodes - 2], path[num_nodes - 1]), 1.0),
            speed,
            acceleration,
            path: Some(path),
        }
    }
}

impl<'a> Instantiable<'a> for CarEntity {
    // NOTE :: a create car is converted to the cartesian referential
    // but a spawned one is already on the cartesian referential
    fn create(&self, world: &mut World, is_rendering_on: bool) {
        let path = if let Some(path) = self.path.clone() {
            path
        } else {
            let lane_graph = world.read_resource::<LaneGraph>();
            let start_node = (self.position.0).0;
            let end_node = (self.destination.0).1;
            let (_cost, path) = lane_graph
                .get_optimal_path_between_nodes(start_node, end_node)
                .unwrap();
            path
        };

        let mut itinerary = Itinerary::new(path);
        itinerary.next();
        let mut entity_builder: EntityBuilder = world
            .create_entity()
            .with(Identifier(self.id.clone()))
            .with(Position {
                val: (self.position.0, Percentage::new_clamp(self.position.1)),
            })
            .with(itinerary)
            .with(Destination {
                val: (
                    self.destination.0,
                    Percentage::new_clamp(self.destination.1),
                ),
            })
            .with(CarType)
            .with(Speed {
                speed: self.speed * MPS,
            });
        if is_rendering_on {
            entity_builder = entity_builder.with(Drawer {
                figure: DrawableShape::Rectangle(Rectangle::new(3.0, 3.0)),
            });
        }
        entity_builder.build();
    }

    fn spawn(
        &self,
        entities: &Entities<'a>,
        updater: &Read<'a, LazyUpdate>,
        is_rendering_on: bool,
    ) {
        let mut itinerary = Itinerary::new(self.path.clone().unwrap());
        itinerary.next();
        let entity = entities.create();
        updater.insert(entity, Identifier(self.id.clone()));
        updater.insert(
            entity,
            Position {
                val: (self.position.0, Percentage::new_clamp(self.position.1)),
            },
        );
        updater.insert(entity, itinerary);
        updater.insert(
            entity,
            Destination {
                val: (
                    self.destination.0,
                    Percentage::new_clamp(self.destination.1),
                ),
            },
        );
        updater.insert(entity, CarType);
        updater.insert(
            entity,
            Speed {
                speed: self.speed * MPS,
            },
        );
        if is_rendering_on {
            updater.insert(
                entity,
                Drawer {
                    figure: DrawableShape::Rectangle(Rectangle::new(3.0, 3.0)),
                },
            );
        }
    }
}

/// for convenience
fn polarfloat_to_cartesian(lat: f64, lon: f64) -> CartesianCoord {
    let polar = PolarCoord::from_float(lat, lon);
    CartesianCoord::from_polar(&polar)
}
