use crate::entities::entity_type::Instantiable;
use crate::entities::types::CarEntity;
use crate::ressources::clock;
use crate::ressources::lane_graph::LaneGraph;
use crate::ressources::lane_graph::NodeId;
use crate::ressources::random::Random;
use crate::simulation::UseDebugger;
use rand::distributions::{Distribution, Normal};
use rand::Rng;
use simumo_derive::simusystem;
use specs::prelude::{Entities, LazyUpdate, Read, ReadExpect, System, Write};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
#[derive(Default)]
pub struct FrequencySpawner {
    pub start_locations: Vec<u64>,
    pub end_locations: Vec<u64>,
    pub min: i32,
    pub max: i32,
}

impl<'a> System<'a> for FrequencySpawner {
    type SystemData = (
        Read<'a, clock::Clock>,
        Write<'a, Random>,
        Entities<'a>,
        ReadExpect<'a, LaneGraph>,
        Read<'a, LazyUpdate>,
        Read<'a, UseDebugger>,
    );

    fn run(
        &mut self,
        (_clock, mut random, entities, lane_graph, updater, use_debugger): Self::SystemData,
    ) {
        let normal_dist = Normal::new(5., 10.);
        let num_cars_to_spawn = random.get_rng().gen_range(self.min, self.max);
        for _ in 1..num_cars_to_spawn {
            let start_node = self.get_random_start_location(&mut random);
            let end_node = self.get_random_end_location(&mut random);
            let speed = normal_dist.sample(random.get_rng());

            let new_car: CarEntity = CarEntity::new(start_node, end_node, speed, 0.0, &lane_graph);
            new_car.spawn(&entities, &updater, use_debugger.0);
        }
    }
}

impl FrequencySpawner {
    pub fn get_random_start_location(&self, random: &mut Random) -> NodeId {
        let pos_n: usize = random.get_rng().gen_range(0, self.start_locations.len());
        self.start_locations[pos_n]
    }

    pub fn get_random_end_location(&self, random: &mut Random) -> NodeId {
        let pos_n: usize = random.get_rng().gen_range(0, self.end_locations.len());
        self.end_locations[pos_n]
    }
}
