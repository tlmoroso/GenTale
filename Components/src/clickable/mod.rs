use specs_derive;
use specs::prelude::*;
use serde::Deserialize;

#[derive(Clone, Copy, Component, Debug, Deserialize)]
pub struct Clickable {
    pub is_clicked: bool,
    pub is_focused: bool,
}