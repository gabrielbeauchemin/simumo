use specs::{Join, Read, ReadStorage, System};

use crate::components::statics::trafficlight::Light;
use crate::components::types::constant::Identifier;
use crate::ressources::clock;
use crate::systems::mobility::MobilitySystem;
use crate::systems::system_type::DispatcherBuilderHook;
use crate::systems::system_type::SystemType;

use simumo_derive::simusystem;
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
pub struct PrintSystem;

impl SystemType for PrintSystem {
    fn setup(self, hook: &mut DispatcherBuilderHook) {
        hook.add(self)
    }

    fn typename() -> String {
        String::from("PrintSystem")
    }

    fn system_name(&self) -> String {
        String::from("PrintSystem")
    }

    fn type_dependencies(&self) -> Vec<String> {
        vec![MobilitySystem::typename()]
    }
}

impl<'a> System<'a> for PrintSystem {
    type SystemData = (
        Read<'a, clock::Clock>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Identifier>,
    );

    fn run(&mut self, (clock, lights, identifiers): Self::SystemData) {
        for (light, id) in (&lights, &identifiers).join() {
            println!("{}: {:#?}, {:#?}", clock.get_time(), id, light);
        }
    }
}
