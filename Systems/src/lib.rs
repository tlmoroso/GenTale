use specs::prelude::*;
use crate::move_player::MovePlayer;

pub mod move_player;
pub mod draw;

// Used as single entry point to running all systems. Should be done in the update portion of the event loop
pub fn run_systems(ecs: &mut World) {
    let mut pm = MovePlayer{};
    pm.run_now(ecs);
    ecs.maintain();
}