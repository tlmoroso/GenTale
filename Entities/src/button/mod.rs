use Components::sprite::{Sprite, SpriteJSON};
use Components::clickable::Clickable;
use Components::physics::{Physics, PhysicsJSON};
use crate::MyEntity;
use specs::{World, WorldExt, Builder};
use warmy::{Store, SimpleKey, Res};
use ggez::Context;
use warmy::json::Json;
use ggez::graphics::{Image, DrawParam, Rect};
use std::path::PathBuf;
use Components::{point_from_slice, rect_from_slice};
use ggez::input::mouse::button_pressed;
use std::borrow::Borrow;
use serde::Deserialize;

const JSON_PATH: &str = "Entities/src/button/button.json";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
struct ButtonJSON {
    sprite: SpriteJSON,
    clickable: Clickable,
    physics: PhysicsJSON
}

pub struct Button {
    sprite: Sprite,
    clickable: Clickable,
    physics: Physics,
}

impl MyEntity for Button {
    fn build_entity(ecs: &mut World, store: &mut Store<Context, SimpleKey>, ctx: &mut Context, json_path: &str) {
        let button_json: Result<Res<ButtonJSON>, _> = store.get_by(&SimpleKey::from_path(json_path), ctx, Json);
        match button_json {
            Ok(button_default) => {
                let button = Button {
                    sprite: Sprite::from(button_default.borrow().sprite.borrow(), ctx),
                    clickable: button_default.borrow().clickable,
                    physics: Physics::from(button_default.borrow().physics.borrow())
                };

                ecs.create_entity()
                    .with(button.physics)
                    .with(button.clickable)
                    .with(button.sprite)
                    .build();
            }
            Err(e) => eprintln!("{}", e)
        }
    }
}