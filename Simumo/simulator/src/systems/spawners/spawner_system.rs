use crate::systems::spawners::FrequencySpawner;
use crate::systems::system_type::DispatcherBuilderHook;
use crate::systems::system_type::SystemType;
use typeinfo::TypeInfo;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SpawnerSystem {
    Frequency(FrequencySpawner),
}
impl SystemType for SpawnerSystem {
    fn setup(self, hook: &mut DispatcherBuilderHook) {
        match self {
            SpawnerSystem::Frequency(s) => hook.add(s),
        }
    }

    fn typename() -> String {
        String::from("SpawnerSystem")
    }

    fn system_name(&self) -> String {
        match self {
            SpawnerSystem::Frequency(s) => String::from(s.type_of()),
        }
    }
}
