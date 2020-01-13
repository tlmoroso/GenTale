extern crate Components;
use Components::Physics::Physics;
use Components::Sprite::Sprite;

use specs::prelude::*;

use crate::MyEntity;

pub struct MainCharacter {
    pub physics: Physics,
    pub sprite: Sprite,
}

impl MyEntity for MainCharacter {
    fn build_entity(self, ecs: &mut World) {
        ecs.create_entity()
            .with(self.physics)
            .with(self.sprite)
            .build();
    }
}