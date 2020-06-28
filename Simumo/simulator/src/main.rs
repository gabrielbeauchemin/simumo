/*!
A simulator for computing data of  traffic routes.
Using this program will allow you to generate metrics of your choice.

For help, add option -h in CLI.
*/

#![allow(dead_code)]
#![allow(clippy::type_complexity)]
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate dimensioned as dim;

mod command_line;
mod commons;
mod components;
mod configurations;
mod entities;
mod osmgraph_api;
mod ressources;
mod simulation;
mod systems;

///Main handle all inputs such as configuration file and CLI arguments then create the simulator.
fn main() {
    env_logger::init();
    let args = command_line::CommandLineArguments::parse();
    let config = configurations::Configuration::from_yaml(&args.configuration_file_path).unwrap();

    let mut simulation = simulation::Simulation::from_config(config);
    simulation.run_simulation();
}
