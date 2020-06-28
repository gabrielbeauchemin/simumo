use crate::systems::clock::standard_clock_system::StandardClockSystem;
use crate::systems::system_type::DispatcherBuilderHook;
use crate::systems::SystemType;
use typeinfo::TypeInfo;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClockSystem {
    StandardClock(StandardClockSystem),
}
impl SystemType for ClockSystem {
    fn setup(self, hook: &mut DispatcherBuilderHook) {
        match self {
            ClockSystem::StandardClock(s) => hook.add(s),
        }
    }

    fn typename() -> String {
        String::from("ClockSystem")
    }

    fn system_name(&self) -> String {
        match self {
            ClockSystem::StandardClock(s) => String::from(s.type_of()),
        }
    }
}
