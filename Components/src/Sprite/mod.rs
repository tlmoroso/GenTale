use specs_derive;
use specs::prelude::*;
use std::path::Path;

use ggez::graphics::Image;

#[derive(Component)]
pub struct Sprite {
    pub image: Image,
}