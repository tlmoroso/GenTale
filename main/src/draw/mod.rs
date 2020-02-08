use specs::prelude::*;

use ggez;
use ggez::graphics::{draw};
use ggez::Context;

extern crate Components;
use Components::physics::Physics;
use Components::sprite::Sprite;

pub fn draw_sprite(ecs: &World, ctx: &mut Context) {
    let physics = ecs.read_storage::<Physics>();
    let sprites = ecs.read_storage::<Sprite>();

    for (phys, sprite) in (&physics, &sprites).join() {
        match draw(ctx, &sprite.image, sprite.draw_params) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e)
        }
    }
}