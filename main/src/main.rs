use std::env;
use std::path;
use ggez;
use ggez::{conf, ContextBuilder, event};

use specs::prelude::*;

use warmy::{Store, StoreOpt};

extern crate Components;
extern crate Entities;
extern crate Systems;

mod draw;
mod input;
mod main_constants;
mod state;
mod scene;
use crate::state::State;

pub fn main() {
    let c = conf::Conf::new();
    let mut ctx = ContextBuilder::new(main_constants::GAME_ID, main_constants::AUTHOR).conf(c);

    // Initialize path to resources directory (must be kept inside /target/debug for some reason) so entities can load sprites from there
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push(main_constants::RESOURCE_PATH);
        ctx = ctx.add_resource_path(path);
    }

    let (ref mut ctx, ref mut event_loop) = ctx
        .build()
        .unwrap();

    let state = &mut State::new(ctx);

    Components::register_components(&mut state.ecs);
    Entities::build_entities(&mut state.ecs, &mut state.store, ctx);

    event::run(ctx, event_loop, state).unwrap();
}