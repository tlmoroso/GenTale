use specs::prelude::*;
use ggez::Context;
use crate::Main_Character::MainCharacter;
use warmy::{Store, SimpleKey};
use crate::button::Button;

pub mod Main_Character;
pub mod button;

pub trait MyEntity {
    fn build_entity(ecs: &mut World, store: &mut Store<Context, SimpleKey>, ctx: &mut Context, json_path: &str);
}

pub fn build_entities(ecs: &mut World, store: &mut Store<Context, SimpleKey>, ctx: &mut Context) {
    MainCharacter::build_entity(ecs, store, ctx);
}