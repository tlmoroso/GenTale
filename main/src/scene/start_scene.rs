use specs::{Dispatcher, World};
use crate::scene::{Scene, Transition};
use ggez::{Context, GameError};
use ggez::event::{KeyMods, KeyCode};
use serde::export::fmt::{Debug, Error};
use serde::export::Formatter;
use Entities::button::Button;

pub struct StartScene<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    buttons: Vec<Button>,
}

impl <'a, 'b> StartScene <'a, 'b> {
    pub fn new(_ctx: &mut Context, ecs: &mut World) -> Self {
    }
}

impl<'a, 'b> Scene for StartScene<'a, 'b> {
    fn update(&mut self, ctx: &mut Context, world: &mut World) -> Result<Transition, String> {
        unimplemented!()
    }

    fn draw(&mut self, ctx: &mut Context, world: &mut World) -> Result<(), GameError> {
        unimplemented!()
    }

    fn key_down_event(&mut self, ctx: &mut Context, key_code: KeyCode, key_mods: KeyMods, repeat: bool, world: &mut World) -> Result<Transition, String> {
        unimplemented!()
    }

    fn name(&self) -> &str {
        unimplemented!()
    }
}

impl<'a, 'b> Debug for StartScene<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}