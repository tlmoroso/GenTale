extern crate Components;
use Components::physics::Physics;
use Components::sprite::Sprite;
use Components::player_movement::PlayerMovement;
use self::Components::player_movement::{WalkingState, MovementStates, STATE};
use self::Components::physics::PhysicsJSON;
use self::Components::sprite::SpriteJSON;

use specs::prelude::*;

use crate::MyEntity;
use ggez::Context;
use ggez::graphics::{Image, DrawParam, Rect};

use warmy::{SimpleKey, Store, Res};
use warmy::json::Json;
use serde::Deserialize;
use std::borrow::Borrow;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MainCharacterJSON {
    pub physics: PhysicsJSON,
    pub sprite: SpriteJSON,
    pub movement: PlayerMovement
}

pub struct MainCharacter {
    pub physics: Physics,
    pub sprite: Sprite,
    pub movement: PlayerMovement,
}

impl MyEntity for MainCharacter {
    fn build_entity(ecs: &mut World, store: &mut Store<Context, SimpleKey>, ctx: &mut Context, json_path: &str) -> Option<Entity> {
        let mc_json: Result<Res<MainCharacterJSON>, _> = store.get_by(&SimpleKey::from_path(json_path), ctx, Json);
        match mc_json {
            Ok(mc_default) => {
                let mc = MainCharacter {
                    physics: Physics::from(mc_default.borrow().physics.borrow()),
                    sprite: Sprite::from(mc_default.borrow().sprite.borrow(), ctx),
                    movement: mc_default.borrow().movement,
                };

                Some(ecs.create_entity()
                    .with(mc.physics)
                    .with(mc.sprite)
                    .with(mc.movement)
                    .build())
            }
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }
}