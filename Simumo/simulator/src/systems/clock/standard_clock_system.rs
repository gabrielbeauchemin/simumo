use crate::ressources;

use simumo_derive::simusystem;
use specs::prelude::{System, Write};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
pub struct StandardClockSystem;
impl<'a> System<'a> for StandardClockSystem {
    type SystemData = Write<'a, ressources::Clock>;

    fn run(&mut self, mut clock: Self::SystemData) {
        clock.update();
    }
}
