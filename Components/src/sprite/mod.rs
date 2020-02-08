use specs_derive;
use specs::prelude::*;
use ggez::graphics::{Image, Rect, DrawParam};
use serde::Deserialize;
use crate::{rect_from_slice, point_from_slice};
use std::borrow::Borrow;
use std::path::PathBuf;
use ggez::Context;

#[derive(Component)]
pub struct Sprite {
    pub image: Image,
    pub draw_params: DrawParam,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DrawParamsJSON {
    pub src: [f32; 4],
    pub dest: [f32; 2]
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SpriteJSON {
    pub image: String,
    pub draw_params: DrawParamsJSON
}

impl Sprite {
    pub fn from(s: &SpriteJSON, ctx: &mut Context) -> Sprite {
        Sprite {
            image: Image::new(ctx, PathBuf::from(s.image.as_str()).as_path()).unwrap(),
            draw_params: DrawParam::new()
                .src(rect_from_slice(s.draw_params.src.borrow()))
                .dest(point_from_slice(s.draw_params.dest.borrow())),
        }
    }
}