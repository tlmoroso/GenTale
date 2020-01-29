#[macro_use]
extern crate specs_derive;

use specs::{World, WorldExt};
use crate::physics::Physics;
use crate::sprite::Sprite;
use crate::player::Player;
use crate::player_movement::PlayerMovement;

pub mod physics;
pub mod sprite;
pub mod player;
pub mod player_movement;

pub fn register_components(ecs: &mut World) {
    ecs.register::<Physics>();
    ecs.register::<Sprite>();
//    ecs.register::<Player>();
    ecs.register::<PlayerMovement>()
}