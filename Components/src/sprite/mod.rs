use specs_derive;
use specs::prelude::*;
use ggez::graphics::{Image, Rect, DrawParam};
use serde::Deserialize;

#[derive(Component)]
pub struct Sprite {
    pub image: Image,
    pub draw_params: DrawParam,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DrawParamsJSON {
    pub src: Vec<f32>,
    pub dest: Vec<f32>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SpriteJSON {
    pub image: String,
    pub draw_params: DrawParamsJSON
}