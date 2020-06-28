/*! Represent systems from the configuration file.*/

use std::collections::HashMap;

use specs::DispatcherBuilder;

use crate::systems::agents::AgentSystem;
use crate::systems::clock::ClockSystem;
use crate::systems::controls::ControlSystem;
use crate::systems::mobility::MobilitySystem;
use crate::systems::physic::PhysicSystem;
use crate::systems::recorders::RecorderSystem;
use crate::systems::spawners::SpawnerSystem;
use crate::systems::SystemType;

#[derive(Deserialize)]
pub struct SystemsConfiguration {
    pub agents: Option<Vec<AgentSystem>>,
    pub clock: ClockSystem,
    pub controls: Option<Vec<ControlSystem>>,
    pub mobility: Option<MobilitySystem>,
    pub physic: Option<PhysicSystem>,
    pub recorders: Option<Vec<RecorderSystem>>,
    pub spawner: Option<SpawnerSystem>,
    //pub others : Vec<UnclassifiedSystem>
}

///todo :: consider if it should be implemented somewhere else
impl SystemsConfiguration {
    pub fn declare_systems(&self, system_mapping: &mut HashMap<String, Vec<String>>) {
        system_mapping.insert(ClockSystem::typename(), vec![self.clock.system_name()]);
        if let Some(agents) = &self.agents {
            system_mapping.insert(AgentSystem::typename(), as_sysname_vec(agents));
        } else {
            system_mapping.insert(AgentSystem::typename(), vec![]);
        }
        if let Some(controls) = &self.controls {
            system_mapping.insert(ControlSystem::typename(), as_sysname_vec(controls));
        } else {
            system_mapping.insert(ControlSystem::typename(), vec![]);
        }
        if let Some(mobility) = &self.mobility {
            system_mapping.insert(MobilitySystem::typename(), vec![mobility.system_name()]);
        } else {
            system_mapping.insert(MobilitySystem::typename(), vec![]);
        }
        if let Some(physic) = &self.physic {
            system_mapping.insert(PhysicSystem::typename(), vec![physic.system_name()]);
        } else {
            system_mapping.insert(PhysicSystem::typename(), vec![]);
        }
        if let Some(recorders) = &self.recorders {
            system_mapping.insert(RecorderSystem::typename(), as_sysname_vec(recorders));
        } else {
            system_mapping.insert(RecorderSystem::typename(), vec![]);
        }
        if let Some(spawner) = &self.spawner {
            system_mapping.insert(SpawnerSystem::typename(), vec![spawner.system_name()]);
        } else {
            system_mapping.insert(SpawnerSystem::typename(), vec![]);
        }
    }

    ///Setup all systems in the simulator
    pub fn setup_systems(
        self,
        builder: &mut DispatcherBuilder,
        systems: &HashMap<String, Vec<String>>,
    ) {
        info!("Setting in dispatcher : clock");
        self.clock.set_in_dispatcher(builder, systems);
        info!("Setting in dispatcher : agents");
        if let Some(agents) = self.agents {
            set_all_in_dispatcher(agents, builder, systems);
        }
        info!("Setting in dispatcher : controls");
        if let Some(controls) = self.controls {
            set_all_in_dispatcher(controls, builder, systems);
        }
        info!("Setting in dispatcher : physic");
        if let Some(physic) = self.physic {
            physic.set_in_dispatcher(builder, systems);
        }
        info!("Setting in dispatcher : mobility");
        if let Some(mobility) = self.mobility {
            mobility.set_in_dispatcher(builder, systems);
        }
        info!("Setting in dispatcher : recorders");
        if let Some(recorders) = self.recorders {
            set_all_in_dispatcher(recorders, builder, systems);
        }
        info!("Setting in dispatcher : logger");
        if let Some(spawner) = self.spawner {
            info!("Setting in dispatcher : spawner");
            spawner.set_in_dispatcher(builder, systems);
        }
    }
}

/// used for convenience to set a system in the simulator dispatcher
fn set_all_in_dispatcher<T: SystemType>(
    systems: Vec<T>,
    builder: &mut DispatcherBuilder,
    sys_mapping: &HashMap<String, Vec<String>>,
) {
    for sys in systems {
        sys.set_in_dispatcher(builder, sys_mapping);
    }
}

/// used for convenience
fn as_sysname_vec<T: SystemType>(systems: &[T]) -> Vec<String> {
    systems.iter().map(|a| a.system_name()).collect()
}
