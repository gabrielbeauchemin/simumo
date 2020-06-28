use crate::systems::controls::ControlSystem;
use crate::systems::physic::AccelerationSystem;
use crate::systems::system_type::DispatcherBuilderHook;
use crate::systems::system_type::SystemType;

use typeinfo::TypeInfo;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum PhysicSystem {
    Acceleration(AccelerationSystem),
}

impl SystemType for PhysicSystem {
    fn setup(self, hook: &mut DispatcherBuilderHook) {
        match self {
            PhysicSystem::Acceleration(s) => hook.add(s),
        }
    }

    fn typename() -> String {
        String::from("PhysicSystem")
    }

    fn system_name(&self) -> String {
        match self {
            PhysicSystem::Acceleration(s) => String::from(s.type_of()),
        }
    }

    fn type_dependencies(&self) -> Vec<String> {
        vec![ControlSystem::typename()]
    }
}
