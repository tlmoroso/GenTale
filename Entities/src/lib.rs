use specs::prelude::*;
use ggez::Context;

pub mod Main_Character;

pub trait MyEntity {
    fn build_entity(ecs: &mut World, ctx: &mut Context);
}