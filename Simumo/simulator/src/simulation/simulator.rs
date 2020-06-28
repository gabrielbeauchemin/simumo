use crate::configurations::generals::EndTime;
use crate::configurations::Configuration;
use crate::entities::entity_type::Instantiable;
use crate::ressources::clock;
use crate::ressources::eventsmanagement::EventsManager;
use crate::ressources::generals::MapBbox;
use crate::ressources::lane_graph::LaneGraph;
use crate::ressources::random::Random;
use crate::simulation::dispatchers::add_ending_systems;
use crate::simulation::dispatchers::add_starting_systems;
use crate::simulation::dispatchers::make_render_dispatcher;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlGraphics;
use piston::event_loop::{EventSettings, Events};
use piston_window::OpenGL;
use piston_window::RenderEvent;
use piston_window::WindowSettings;
use rts_logger::LogWriterManager;
use specs::prelude::{DispatcherBuilder, World};
use specs::Dispatcher;
use std::collections::HashMap;
use uuid::Uuid;
use crate::ressources::random_speed::RandomSpeed;
//use std::process::Command;

pub struct UseDebugger(pub bool);
impl Default for UseDebugger {
    fn default() -> Self {
        Self(false)
    }
}

pub struct Simulation<'a, 'b> {
    world: World,
    base_dispatcher: Dispatcher<'a, 'b>,
    rendering: (bool, Dispatcher<'a, 'b>),
    window: Option<Window>,
    loggers: Option<LogWriterManager>,
}

impl<'a, 'b> Simulation<'a, 'b> {
    const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

    pub fn from_config(config: Configuration) -> Self {
        let loggers = match &config.generals.logging {
            Some(logging) => Some(logging.get_manager()),
            None => None,
        };

        let mut base_dispatcher_builder = DispatcherBuilder::new();
        let mut world = World::new();
        let mut system_mapping = HashMap::<String, Vec<String>>::new();

        let is_rendering_on: bool = config.generals.debugger.on;
        let width: f64 = config.generals.debugger.width;
        let height: f64 = config.generals.debugger.height;
        let window = if is_rendering_on {
            Some(Self::create_window(width, height))
        } else {
            None
        };
        Self::create_ressources(&mut world, &config);

        config.systems.declare_systems(&mut system_mapping);
        add_starting_systems(&mut base_dispatcher_builder);
        config
            .systems
            .setup_systems(&mut base_dispatcher_builder, &system_mapping);
        add_ending_systems(&mut base_dispatcher_builder);

        let mut base_dispatcher = base_dispatcher_builder.build();
        base_dispatcher.setup(&mut world.res);

        let rendering = if is_rendering_on {
            let mut render_dispatcher = make_render_dispatcher();
            render_dispatcher.setup(&mut world.res);
            (is_rendering_on, render_dispatcher)
        } else {
            (is_rendering_on, DispatcherBuilder::new().build())
        };
        world.add_resource(UseDebugger(is_rendering_on));

        //entities
        if let Some(entities) = config.entities {
            for entity in entities.iter() {
                entity.create(&mut world, is_rendering_on);
            }
        }

        Self {
            world,
            base_dispatcher,
            window,
            rendering,
            loggers,
        }
    }

    pub fn run_simulation(&mut self) {
        let mut events = Events::new(EventSettings::new());
        let is_render_on = self.rendering.0;
        let mut is_simulation_running = true;
        while should_keep_going(is_render_on, is_simulation_running) {
            if !simulation_ended(&self.world) {
                is_simulation_running = true;
                self.base_dispatcher.dispatch(&self.world.res);
                self.world.maintain();
            } else {
                is_simulation_running = false;
            }
            if is_render_on {
                if let Some(ref mut window) = &mut self.window {
                    if let Some(e) = events.next(&mut *window) {
                        if let Some(r) = e.render_args() {
                            self.world.add_resource(r);
                            self.rendering.1.dispatch(&self.world.res);
                            self.world.maintain();
                        }
                    }
                };
            }
        }
        println!("Showing results log...");
    }

    fn create_window(width: f64, height: f64) -> Window {
        WindowSettings::new("Simumo - Visual debugger", [width, height])
            .opengl(Self::OPENGL_VERSION)
            .exit_on_esc(true)
            .build()
            .unwrap()
    }
    //
    ///Create default world's ressources and config's ressources
    fn create_ressources(world: &mut World, config: &Configuration) {
        let end_time = config.generals.end_time.clone();
        let seed = if !config.generals.seed.is_empty() {
            Uuid::parse_str(&config.generals.seed).unwrap_or_else(|_| panic!("invalid seed format"))
        } else {
            Uuid::new_v4()
        };
        let random = Random::from_uuid(&seed);
        let (lane_graph, bbox): (LaneGraph, MapBbox) = config.map.forward_ressources();

        if config.generals.debugger.on {
            let graphics_handle = GlGraphics::new(Self::OPENGL_VERSION);
            let debugger = config.generals.debugger.clone();
            world.add_resource(graphics_handle);
            world.add_resource(debugger);
            world.add_resource(bbox);
        }
        world.add_resource(lane_graph);
        world.add_resource(end_time);
        world.add_resource(clock::Clock::new(config.generals.clock_dt));
        world.add_resource(EventsManager::new());
        world.add_resource(random);
        if let Some(random_speed) = config.generals.random_speed{
            world.add_resource(RandomSpeed(random_speed) );
        }
        else {
            world.add_resource(RandomSpeed(false) );
        }

    }
}

fn should_keep_going(_is_render_on: bool, is_simulation_running: bool) -> bool {
    // note :: this is commented because of the python bug
    //  basically we cant Ctrl+C the process while OpenGL + python is running in the same time
    //  todo :: find a fix for the problem above
    //if is_render_on {
    //    return true;
    //}
    is_simulation_running
}

fn simulation_ended(ressources: &World) -> bool {
    // if keyboard end event  +
    let clock = ressources.read_resource::<clock::Clock>();
    let end_time = ressources.read_resource::<EndTime>();
    clock.get_time() >= end_time.val
}
