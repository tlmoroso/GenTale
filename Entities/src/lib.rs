use specs::prelude::*;
use ggez::Context;
use warmy::{Store, SimpleKey};

pub mod main_character;
pub use self::main_character::MainCharacter;
pub mod button;
pub use self::button::Button;

pub trait MyEntity {
    fn build_entity(ecs: &mut World, store: &mut Store<Context, SimpleKey>, ctx: &mut Context, json_path: &str) -> Option<Entity>;
}