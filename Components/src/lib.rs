#[macro_use]
extern crate specs_derive;

use specs::{World, WorldExt};
use crate::physics::Physics;
use crate::sprite::Sprite;
use crate::player::Player;
use crate::player_movement::PlayerMovement;
use crate::clickable::Clickable;
use mint::Point2;
use ggez::graphics::{Rect};

pub mod physics;
pub mod sprite;
pub mod player;
pub mod clickable;
pub mod errors;
pub mod player_movement;

pub fn register_components(ecs: &mut World) {
    ecs.register::<Physics>();
    ecs.register::<Sprite>();
    ecs.register::<Clickable>();
//    ecs.register::<Player>();
    ecs.register::<PlayerMovement>()
}

pub fn rect_from_slice(vec: &[f32; 4]) -> Rect {
    Rect::new(vec[0], vec[1], vec[2], vec[3])
}

pub fn point_from_slice(vec: &[f32; 2]) -> Point2<f32> {
    Point2::from_slice(vec)
}