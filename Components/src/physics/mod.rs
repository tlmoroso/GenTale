use specs_derive;
use specs::prelude::*;
use mint::Point2;
use serde::Deserialize;

#[derive(Component)]
pub struct Physics {
    pub position: Point2<f32>,
    pub velocity: f32,
    pub acceleration: f32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PhysicsJSON {
    pub position: Vec<f32>,
    pub velocity: f32,
    pub acceleration: f32
}