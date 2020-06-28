use crate::components::types::constant::Identifier;
use crate::ressources::eventsmanagement::EventsManager;
use specs::prelude::{Entities, ReadStorage, System, Write};

pub struct EventsHookUpdate;
impl<'a> System<'a> for EventsHookUpdate {
    type SystemData = (
        Write<'a, EventsManager>,
        Entities<'a>,
        ReadStorage<'a, Identifier>,
    );

    fn run(&mut self, (_eventsmanager, _entities, _identifiers): Self::SystemData) {
        /*for (entity, identifier, observableslist) in (&entities, &identifiers, &mut observables).join() {
            let currentObservables: &Vec<&Entity> = observableslist.get_list();
            for currentObservable in currentObservables {

            }
        }*/
    }
}

pub struct EventsUpdate;
impl<'a> System<'a> for EventsUpdate {
    type SystemData = (Write<'a, EventsManager>);

    fn run(&mut self, mut eventsmanager: Self::SystemData) {
        eventsmanager.swap_events();
    }
}
