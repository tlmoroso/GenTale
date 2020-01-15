use std::cmp::{min, max};

use specs::prelude::*;

use ggez::Context;
use ggez::input::keyboard;
use ggez::event::KeyCode;

extern crate Components;
use Components::Physics::Physics;
use Components::Player::Player;


fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut physics = ecs.write_storage::<Physics>();
    let mut players = ecs.read_storage::<Player>();

    for (_player, phys) in (& players, &mut physics).join() {
        phys.position.x = max(0,min(800,phys.position.x + delta_x));
        phys.position.y = max(0,min(600,phys.position.y + delta_y));
    }
}

pub fn player_input(ecs: &mut World, ctx: &mut Context) {
    // Player movement
    for key in keyboard::pressed_keys(ctx).iter() {
        match key {
            KeyCode::Left => try_move_player(-1, 0, ecs),
            KeyCode::Right => try_move_player(1, 0, ecs),
            KeyCode::Up => try_move_player(0, -1, ecs),
            KeyCode::Down => try_move_player(0, 1, ecs),
            _ => {}
        }
    }
}