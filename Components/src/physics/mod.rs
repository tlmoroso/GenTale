use specs_derive;
use specs::prelude::*;
use mint::Point2;
use serde::Deserialize;
use crate::point_from_slice;
use std::borrow::Borrow;

#[derive(Component)]
pub struct Physics {
    pub position: Point2<f32>,
    pub velocity: f32,
    pub acceleration: f32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PhysicsJSON {
    pub position: [f32; 2],
    pub velocity: f32,
    pub acceleration: f32
}

impl Physics {
    pub fn from(p: &PhysicsJSON) -> Physics {
        Physics {
            position: point_from_slice(p.position.borrow()),
            velocity: p.velocity,
            acceleration: p.acceleration
        }
    }
}