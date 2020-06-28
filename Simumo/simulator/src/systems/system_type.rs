use std::collections::HashMap;

use specs::DispatcherBuilder;
use specs::System;

type SystemMapping = HashMap<String, Vec<String>>;

pub trait SystemType {
    fn setup(self, hook: &mut DispatcherBuilderHook);

    fn set_in_dispatcher(
        self,
        dispatcher_builder: &mut DispatcherBuilder,
        dependency_mapping: &SystemMapping,
    ) where
        Self: Sized,
    {
        let mut type_dep = self
            .type_dependencies()
            .iter()
            .flat_map(|s| dependency_mapping.get(s).unwrap().clone())
            .collect::<Vec<String>>();

        let sys_dep = self
            .system_dependencies()
            .values()
            .flat_map(|d| d.clone())
            .collect::<Vec<String>>();

        type_dep.extend(sys_dep);
        let mut hook = DispatcherBuilderHook {
            name: self.system_name(),
            dependencies: type_dep,
            dispatcher: dispatcher_builder,
        };
        self.setup(&mut hook);
    }

    fn typename() -> String
    where
        Self: Sized;

    fn system_name(&self) -> String;

    fn type_dependencies(&self) -> Vec<String> {
        Vec::new()
    }
    fn system_dependencies(&self) -> SystemMapping {
        SystemMapping::new()
    }
}

///
///
///
pub struct DispatcherBuilderHook<'a, 'b, 'c> {
    name: String,
    dependencies: Vec<String>,
    dispatcher: &'a mut DispatcherBuilder<'b, 'c>,
}

impl<'a, 'b, 'c> DispatcherBuilderHook<'a, 'b, 'c> {
    pub fn add<T>(&mut self, system: T)
    where
        T: for<'f> System<'f> + Send + 'b,
    {
        self.dispatcher.add(
            system,
            &*self.name,
            self.dependencies
                .iter()
                .map(|s| &**s)
                .collect::<Vec<&str>>()
                .as_slice(),
        )
    }
}
