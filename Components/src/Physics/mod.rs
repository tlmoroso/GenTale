use specs_derive;
use specs::prelude::*;

#[derive(Component)]
pub struct Physics {
    position: Position,
    velocity: i32,
    acceleration: i32
}

struct Position {
    x: u32,
    y: u32,
}