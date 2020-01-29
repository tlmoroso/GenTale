use specs::prelude::*;

use ggez;
use ggez::graphics::{Image, draw, DrawParam, Rect};
use ggez::nalgebra::Point2;
use ggez::Context;

extern crate Components;
use Components::physics::Physics;
use Components::sprite::Sprite;
use Components::player_movement::PlayerMovement;

pub fn draw_sprite(ecs: &World, ctx: &mut Context) {
    let physics = ecs.read_storage::<Physics>();
    let sprites = ecs.read_storage::<Sprite>();

    for (phys, sprite) in (&physics, &sprites).join() {
        draw(ctx, &sprite.image, sprite.draw_params);
    }
}