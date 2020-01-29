use Components::physics::Physics;
use specs::{World, WorldExt, ReadStorage, WriteStorage, System, ReadExpect};
use Components::player::Player;
use std::cmp::{max, min};
use Components::player_movement::{MovementStates, STATE, PlayerMovement};
use ggez::Context;
use ggez::input::keyboard::KeyCode;
use std::collections::hash_set::Iter;
use std::collections::HashSet;
use specs::join::Join;
use Components::sprite::Sprite;

pub struct MovePlayer {}

impl<'a> System<'a> for MovePlayer {
    type SystemData = (WriteStorage<'a, PlayerMovement>,
                       WriteStorage<'a, Physics>,
                       WriteStorage<'a, Sprite>,
                       ReadExpect<'a, Vec<KeyCode>>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut player_movement,
             mut physics,
             mut sprite,
                 keys_pressed) = data;

        for (mut pm, mut phys, mut spr) in (&mut player_movement, &mut physics, &mut sprite).join() {
            match pm.current_state {
                STATE::WalkingState => {
                    match pm.states.walking_state.update(&keys_pressed, &mut phys, &mut spr) {
                        Some(new_state) => pm.current_state = new_state,
                        None => (),
                    }
                },
                _ => (),
            }
        }
    }
}