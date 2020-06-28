use crate::systems::agents::AgentSystem;
use crate::systems::controls::LightControl;
use crate::systems::system_type::DispatcherBuilderHook;
use crate::systems::system_type::SystemType;
use typeinfo::TypeInfo;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ControlSystem {
    Light(LightControl),
}
impl SystemType for ControlSystem {
    fn setup(self, hook: &mut DispatcherBuilderHook) {
        match self {
            ControlSystem::Light(s) => hook.add(s),
        }
    }

    fn typename() -> String {
        String::from("ControlSystem")
    }

    fn system_name(&self) -> String {
        match self {
            ControlSystem::Light(s) => String::from(s.type_of()),
        }
    }

    fn type_dependencies(&self) -> Vec<String> {
        vec![AgentSystem::typename()]
    }
}
