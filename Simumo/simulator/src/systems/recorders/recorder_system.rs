use crate::systems::mobility::MobilitySystem;
use crate::systems::recorders::CarPositionRecorderSystem;
use crate::systems::system_type::DispatcherBuilderHook;
use crate::systems::system_type::SystemType;

use crate::systems::recorders::CarSpeedRecorderSystem;
use typeinfo::TypeInfo;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum RecorderSystem {
    CarPositionRecorder(CarPositionRecorderSystem),
    CarSpeedRecorder(CarSpeedRecorderSystem),
}
impl SystemType for RecorderSystem {
    fn setup(self, hook: &mut DispatcherBuilderHook) {
        match self {
            RecorderSystem::CarPositionRecorder(s) => hook.add(s),
            RecorderSystem::CarSpeedRecorder(s) => hook.add(s),
        }
    }

    fn typename() -> String {
        String::from("RecorderSystem")
    }
    fn system_name(&self) -> String {
        match self {
            RecorderSystem::CarPositionRecorder(s) => String::from(s.type_of()),
            RecorderSystem::CarSpeedRecorder(s) => String::from(s.type_of()),
        }
    }

    fn type_dependencies(&self) -> Vec<String> {
        vec![MobilitySystem::typename()]
    }
}
