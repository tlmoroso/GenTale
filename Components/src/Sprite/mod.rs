use specs_derive;
use specs::prelude::*;
use ggez::graphics::Image;

#[derive(Component)]
pub struct Sprite {
    pub image: Image,
}