use std::cmp::{max, min};

use specs::prelude::*;

use ggez::Context;
use ggez::input::keyboard;
use ggez::event::KeyCode;

use crate::physics::Physics;
use std::collections::HashSet;
use crate::sprite::Sprite;
use mint::Point2;
use ggez::graphics::Rect;

use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum STATE {
    WalkingState,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WalkingState {
    pub WALK_CYCLE: u32, // constant
    pub FRAME_COUNT: u32,
    pub current_cycle: u32,
    pub frame_counter: u32,
}

impl WalkingState {
    fn try_move_player(&mut self, delta_x: f32, delta_y: f32, physics: &mut Physics) {
        physics.position.x = (0.0 as f32).max((800.0 as f32).min(physics.position.x + delta_x));
        physics.position.y = (0.0 as f32).max((600.0 as f32).min(physics.position.y + delta_y));
    }

    fn update_sprite(&mut self, physics: &mut Physics, sprite: &mut Sprite) {
        sprite.draw_params.dest = Point2::from_slice(vec!(physics.position.x, physics.position.y).as_slice());

        let x_top_left: f32 = 0.0;
        let y_top_left: f32 = (self.current_cycle as f32)/(self.WALK_CYCLE as f32);
        let width: f32 = 1.0;
        let height: f32 = (1.0/(self.WALK_CYCLE as f32)) - ((1.0/(self.WALK_CYCLE as f32)) / 256.0);
        sprite.draw_params.src = Rect::new(x_top_left, y_top_left, width, height);
    }

    fn handle_input(&mut self, keys_pressed: &Vec<KeyCode>, physics: &mut Physics, sprite: &mut Sprite) -> Option<STATE> {
        let mut delta_x: f32 = 0.0;
        let mut delta_y: f32 = 0.0;

        for key in keys_pressed {
            match key {
                KeyCode::Left => {
                    delta_x = delta_x - 1.0;
                },
                KeyCode::Right => {
                    delta_x = delta_x + 1.0;
                },
                KeyCode::Up => {
                    delta_y = delta_y - 1.0;
                },
                KeyCode::Down => {
                    delta_y = delta_y + 1.0;
                },
                _ => {}
            }
        }
        if delta_y != 0.0 || delta_x != 0.0 {
            self.try_move_player(delta_x, delta_y, physics);
            self.frame_counter = self.frame_counter + 1;
            self.current_cycle = ((self.frame_counter)/self.FRAME_COUNT) % self.WALK_CYCLE;
            self.update_sprite(physics, sprite);
        }

        return None;
    }

    pub fn update(&mut self, keys_pressed: &Vec<KeyCode>, physics: &mut Physics, sprite: &mut Sprite) -> Option<STATE> {
        self.handle_input(keys_pressed, physics, sprite) // Lack of semicolon specifies that the return value of handle_input is also the return value of update
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MovementStates {
    pub walking_state: WalkingState,
}


#[derive(Clone, Component, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlayerMovement {
    pub states: MovementStates,
    pub current_state: STATE,
}
