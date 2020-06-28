use crate::systems::agents::AcceleratingAgentSystem;
use crate::systems::clock::ClockSystem;
use crate::systems::system_type::DispatcherBuilderHook;
use crate::systems::SystemType;
use typeinfo::TypeInfo;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum AgentSystem {
    Accelerating(AcceleratingAgentSystem),
}

impl SystemType for AgentSystem {
    fn setup(self, hook: &mut DispatcherBuilderHook) {
        match self {
            AgentSystem::Accelerating(s) => hook.add(s),
        }
    }

    fn typename() -> String {
        String::from("AgentSystem")
    }

    fn system_name(&self) -> String {
        match self {
            AgentSystem::Accelerating(s) => String::from(s.type_of()),
        }
    }

    fn type_dependencies(&self) -> Vec<String> {
        vec![ClockSystem::typename()]
    }
}
