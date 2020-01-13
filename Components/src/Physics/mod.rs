use specs_derive;
use specs::prelude::*;
use ggez::nalgebra::Point2;

#[derive(Component)]
pub struct Physics {
    pub position: Point2<f32>,
    pub velocity: i32,
    pub acceleration: i32
}