use std::env;
use std::path;
use std::path::PathBuf;
use std::borrow::Borrow;

use ggez;
use ggez::{conf, ContextBuilder, GameResult, event, Context, graphics};
use ggez::filesystem::resources_dir;

use specs::prelude::*;

use warmy::{Store, StoreOpt, SimpleKey};

extern crate Components;
use Components::physics::*;
use Components::sprite::Sprite;
use Components::player::Player;

extern crate Entities;
use Entities::Main_Character::MainCharacter;
use Entities::{MyEntity};
use ggez::input::keyboard;
use std::collections::HashSet;
use ggez::event::KeyCode;

extern crate Systems;

mod draw;
mod input;
use crate::input::get_current_input;
mod main_constants;
pub mod state;
use crate::state::State;

pub fn main() {
    let c = conf::Conf::new();
    let mut ctx = ContextBuilder::new(main_constants::GAME_ID, main_constants::AUTHOR).conf(c);
    let mut store = Store::new(StoreOpt::default()).unwrap();

    // Initialize path to resources directory (must be kept inside /target/debug for some reason) so entities can load sprites from there
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push(main_constants::RESOURCE_PATH);
        ctx = ctx.add_resource_path(path);
    }

    let (ref mut ctx, ref mut event_loop) = ctx
        .build()
        .unwrap();

    let state = &mut State {
        ecs   : World::new(),
        store,
    };

    Components::register_components(&mut state.ecs);
    Entities::build_entities(&mut state.ecs, &mut state.store, ctx);

    event::run(ctx, event_loop, state).unwrap();
}