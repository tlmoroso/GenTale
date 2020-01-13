use specs::prelude::*;

pub mod Main_Character;

pub trait MyEntity {
    fn build_entity(self, ecs: &mut World);
}