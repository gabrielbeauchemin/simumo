use crate::systems::mobility::StandardMobilitySystem;
use crate::systems::physic::PhysicSystem;
use crate::systems::system_type::DispatcherBuilderHook;
use crate::systems::system_type::SystemType;

use typeinfo::TypeInfo;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum MobilitySystem {
    StandardMobility(StandardMobilitySystem),
}

impl SystemType for MobilitySystem {
    fn setup(self, hook: &mut DispatcherBuilderHook) {
        match self {
            MobilitySystem::StandardMobility(s) => hook.add(s),
        }
    }

    fn typename() -> String {
        String::from("MobilitySystem")
    }

    fn system_name(&self) -> String {
        match self {
            MobilitySystem::StandardMobility(s) => String::from(s.type_of()),
        }
    }

    fn type_dependencies(&self) -> Vec<String> {
        vec![PhysicSystem::typename()]
    }
}
