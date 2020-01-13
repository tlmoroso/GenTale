extern crate Components;
use Components::Physics::Physics;
use Components::Sprite::Sprite;

use specs::prelude::*;

use crate::MyEntity;
use ggez::Context;
use ggez::graphics::Image;
use ggez::nalgebra::Point2;

use std::path::PathBuf;

pub struct MainCharacter {
    pub physics: Physics,
    pub sprite: Sprite,
}

impl MyEntity for MainCharacter {
    fn build_entity(ecs: &mut World, ctx: &mut Context) {
        let mc = MainCharacter {
            physics: Physics {
                position: Point2::new(100.0, 100.0),
                velocity: 0,
                acceleration: 0
            },
            sprite: Sprite {
                image: Image::new(ctx, PathBuf::from("\\MainCharacter\\main_character_3.png").as_path()).unwrap()
            }
        };

        ecs.create_entity()
            .with(mc.physics)
            .with(mc.sprite)
            .build();
    }
}