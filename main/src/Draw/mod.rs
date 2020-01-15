use specs::prelude::*;

use ggez;
use ggez::graphics::{Image, draw, DrawParam};
use ggez::nalgebra::Point2;
use ggez::Context;

extern crate Components;
use Components::Physics::Physics;
use Components::Sprite::Sprite;

pub fn draw_sprite(ecs: &World, ctx: &mut Context) {
    let physics = ecs.read_storage::<Physics>();
    let sprites = ecs.read_storage::<Sprite>();

    for (phys, sprite) in (&physics, &sprites).join() {
        draw(ctx, &sprite.image, DrawParam::default().dest(Point2::new(phys.position.x as f32, phys.position.y as f32)));
    }
}