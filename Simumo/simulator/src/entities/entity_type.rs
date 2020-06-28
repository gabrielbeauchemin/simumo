use crate::entities::types::CarEntity;
use crate::entities::types::LightEntity;
use specs::prelude::{Entities, LazyUpdate, Read, World};

pub trait Instantiable<'a> {
    fn create(&self, world: &mut World, is_rendering_on: bool);
    fn spawn(&self, entities: &Entities<'a>, updater: &Read<'a, LazyUpdate>, is_rendering_on: bool);
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EntityType {
    #[serde(rename = "vehicle")]
    CarEntity(CarEntity),
    #[serde(rename = "trafficlight")]
    LightEntity(LightEntity),
}

impl<'a> Instantiable<'a> for EntityType {
    fn create(&self, world: &mut World, is_rendering_on: bool) {
        match self {
            EntityType::CarEntity(car) => car.create(world, is_rendering_on),
            EntityType::LightEntity(light) => light.create(world, is_rendering_on),
        }
    }

    fn spawn(
        &self,
        entities: &Entities<'a>,
        updater: &Read<'a, LazyUpdate>,
        is_rendering_on: bool,
    ) {
        match self {
            EntityType::CarEntity(car) => car.spawn(entities, updater, is_rendering_on),
            EntityType::LightEntity(light) => light.spawn(entities, updater, is_rendering_on),
        }
    }
}
