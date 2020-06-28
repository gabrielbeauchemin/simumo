use crate::components::agents::AcceleratingAgent;
use crate::components::controls::EnergyControl;
use crate::components::types::constant::CarType;
use simumo_derive::simusystem;
use specs::prelude::{Join, ReadStorage, System, WriteStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
pub struct AcceleratingAgentSystem;
impl<'a> System<'a> for AcceleratingAgentSystem {
    type SystemData = (
        ReadStorage<'a, CarType>,
        WriteStorage<'a, AcceleratingAgent>,
        WriteStorage<'a, EnergyControl>,
    );

    fn run(&mut self, (car_types, mut agnts, mut ctrls): Self::SystemData) {
        for (_car_type, mut agnt, mut ctrl) in (&car_types, &mut agnts, &mut ctrls).join() {
            if ctrl.0 > 100 {
                agnt.is_decelerating = true;
            } else if ctrl.0 < -100 {
                agnt.is_decelerating = false;
            }

            if agnt.is_decelerating {
                ctrl.0 -= 1
            } else {
                ctrl.0 += 1
            }
        }
    }
}
