use crate::components::dynamic::{Acceleration, Speed};
use crate::ressources::Clock;

use simumo_derive::simusystem;
use specs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
pub struct AccelerationSystem;
impl<'a> System<'a> for AccelerationSystem {
    type SystemData = (
        WriteStorage<'a, Speed>,
        ReadStorage<'a, Acceleration>,
        Read<'a, Clock>,
    );

    fn run(&mut self, (mut vel, acc, clock): Self::SystemData) {
        for (vel, acc) in (&mut vel, &acc).join() {
            vel.speed += acc.val * clock.dt;
        }
    }
}
