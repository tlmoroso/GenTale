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
use mint::Point2;

use std::path::PathBuf;

use warmy::{SimpleKey, StoreOpt, Store, Res};
use warmy::json::Json;
use serde::Deserialize;

const SPRITE_PATH: &str = "\\MainCharacter\\main_character.png";

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
    fn build_entity(ecs: &mut World, store: &mut Store<Context, SimpleKey>, ctx: &mut Context) {
        let mc_json: Result<Res<MainCharacterJSON>, _> = store.get_by(&SimpleKey::from_path("Entities/src/Main_Character/main_character.json"), ctx, Json);
        match mc_json {
            Ok(mc_default) => {
                let mc = MainCharacter {
                    physics: Physics {
                        position: Point2::from_slice(vec!(mc_default.borrow().physics.position[0],
                                                          mc_default.borrow().physics.position[1]).as_slice()),
                        velocity: mc_default.borrow().physics.velocity,
                        acceleration: mc_default.borrow().physics.acceleration
                    },
                    sprite: Sprite {
                        image: Image::new(ctx, PathBuf::from(mc_default.borrow().sprite.image.as_str()).as_path()).unwrap(),
                        draw_params: DrawParam::new()
                            .src(Rect::new(mc_default.borrow().sprite.draw_params.src[0],
                                           mc_default.borrow().sprite.draw_params.src[1],
                                           mc_default.borrow().sprite.draw_params.src[2],
                                           mc_default.borrow().sprite.draw_params.src[3]))
                            .dest(Point2::from_slice(vec!(mc_default.borrow().sprite.draw_params.dest[0],
                                                          mc_default.borrow().sprite.draw_params.dest[1]).as_slice())),
                    },
                    movement: mc_default.borrow().movement,
                };

                ecs.create_entity()
                    .with(mc.physics)
                    .with(mc.sprite)
                    .with(mc.movement)
                    .build();
            }

            Err(e) => eprintln!("{}", e)
        }
    }
}