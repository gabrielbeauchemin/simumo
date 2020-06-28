use crate::commons::CartesianCoord;
use crate::commons::Percentage;
use crate::commons::PolarCoord;
use crate::components::types::constant::Drawer;
use crate::components::types::constant::Identifier;
use crate::components::types::statics::trafficlight::Light;
use crate::components::Position;
use crate::entities::entity_type::Instantiable;
use crate::ressources::eventsmanagement::EventsManager;
use crate::ressources::lane_graph::NodeId;
use crate::systems::renderer::drawableshape::Circle;
use crate::systems::renderer::drawableshape::DrawableShape;
use specs::prelude::{Entities, LazyUpdate, Read};
use specs::Builder;
use specs::EntityBuilder;
use specs::World;

#[derive(Deserialize, Debug)]
pub struct LightEntity {
    pub id: String,
    //todo :: we should make a deserializable light
    // it would split the behaviour of the config and the simulation
    pub light: Light,
    #[serde(default)]
    pub position: ((NodeId, NodeId), f64),
    pub observable: String,
}

impl LightEntity {
    fn connect_to_observable(&self, world: &mut World, id_observable: String) {
        let mut events_manager = world.write_resource::<EventsManager>();
        events_manager.connect(id_observable, self.id.clone());
    }
}

impl<'a> Instantiable<'a> for LightEntity {
    fn create(&self, world: &mut World, is_rendering_on: bool) {
        self.connect_to_observable(world, self.observable.clone());
        let mut entity_builder: EntityBuilder = world
            .create_entity()
            .with(Identifier(self.id.clone()))
            .with(self.light)
            .with(Position {
                val: (self.position.0, Percentage::new_clamp(self.position.1)),
            });
        if is_rendering_on {
            entity_builder = entity_builder.with(Drawer {
                figure: DrawableShape::Circle(Circle::new(4.0)),
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
        let entity = entities.create();
        updater.insert(entity, Identifier(self.id.clone()));
        updater.insert(entity, self.light);
        updater.insert(
            entity,
            Position {
                val: (self.position.0, Percentage::new_clamp(self.position.1)),
            },
        );
        if is_rendering_on {
            updater.insert(
                entity,
                Drawer {
                    figure: DrawableShape::Circle(Circle::new(4.0)),
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
