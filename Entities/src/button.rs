use Components::sprite::{Sprite, SpriteJSON};
use Components::clickable::Clickable;
use Components::physics::{Physics, PhysicsJSON};
use crate::MyEntity;
use specs::{World, WorldExt, Builder, Entity};
use warmy::{Store, SimpleKey, Res};
use ggez::Context;
use warmy::json::Json;
use std::borrow::Borrow;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
struct ButtonJSON {
    sprite: SpriteJSON,
    clickable: Clickable,
    physics: PhysicsJSON
}

#[derive(Debug)]
pub struct Button {
    sprite: Sprite,
    clickable: Clickable,
    physics: Physics,
}

impl MyEntity for Button {
    fn build_entity(ecs: &mut World, store: &mut Store<Context, SimpleKey>, ctx: &mut Context, json_path: &str) -> Option<Entity> {
        let button_json: Result<Res<ButtonJSON>, _> = store.get_by(&SimpleKey::from_path(json_path), ctx, Json);
        match button_json {
            Ok(button_default) => {
                let button = Button {
                    sprite: Sprite::from(button_default.borrow().sprite.borrow(), ctx),
                    clickable: button_default.borrow().clickable,
                    physics: Physics::from(button_default.borrow().physics.borrow())
                };

                Some(ecs.create_entity()
                    .with(button.physics)
                    .with(button.clickable)
                    .with(button.sprite)
                    .build())
            }
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }
}