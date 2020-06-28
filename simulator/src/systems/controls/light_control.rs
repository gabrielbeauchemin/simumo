use crate::components::types::constant::Identifier;
use crate::components::types::statics::trafficlight::Light;
use crate::components::types::statics::trafficlight::TrafficLightColor;
use crate::ressources::clock;
use crate::ressources::eventsmanagement::{Event, EventsManager};

use dim::si::S;
use simumo_derive::simusystem;
use specs::prelude::{Join, Read, ReadStorage, System, Write, WriteStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
pub struct LightControl;
impl<'a> System<'a> for LightControl {
    type SystemData = (
        Write<'a, EventsManager>,
        ReadStorage<'a, Identifier>,
        WriteStorage<'a, Light>,
        Read<'a, clock::Clock>,
    );

    fn run(&mut self, (mut eventsmanager, identifiers, mut lights, clock): Self::SystemData) {
        for (identifier, light) in (&identifiers, &mut lights).join() {
            // We check the events that apply (the events that were triggered by the entities that are observed by this one)
            let events_to_execute: Vec<&Event> =
                eventsmanager.get_events_to_execute(identifier.0.as_str());
            for event_to_execute in events_to_execute.iter() {
                match event_to_execute {
                    Event::TrafficLightColorChange(new_color) => {
                        if new_color == &TrafficLightColor::RED {
                            light.reset_to_green();
                        }
                    }
                }
            }
            // We update the light's time (and color if applicable)
            match light.color {
                TrafficLightColor::GREEN => {
                    light.time -= clock.get_dt();
                    if light.time <= (core::f64::EPSILON * S) {
                        light.reset_to_yellow();
                        eventsmanager.add_event_to_be_executed(
                            identifier.0.as_str(),
                            &Event::TrafficLightColorChange(TrafficLightColor::YELLOW),
                        );
                    }
                }
                TrafficLightColor::YELLOW => {
                    light.time -= clock.get_dt();
                    if light.time <= (core::f64::EPSILON * S) {
                        light.reset_to_red();
                        eventsmanager.add_event_to_be_executed(
                            identifier.0.as_str(),
                            &Event::TrafficLightColorChange(TrafficLightColor::RED),
                        )
                    }
                }
                _ => (),
            }
        }
    }
}
