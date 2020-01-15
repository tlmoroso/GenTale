use std::env;
use std::path;
use std::path::PathBuf;

use ggez;
use ggez::{conf, ContextBuilder, GameResult, event, Context, graphics};
use ggez::filesystem::resources_dir;

use specs::prelude::*;

extern crate Components;
use Components::Physics::*;
use Components::Sprite::Sprite;
use Components::Player::Player;

extern crate Entities;
use Entities::Main_Character::MainCharacter;
use Entities::{MyEntity};

mod Draw;
mod Input;

struct State {
    ecs:    World,
}

impl State {
    // Used as single entry point to running all systems. Should be done in the update portion of the event loop
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Input::player_input(&mut self.ecs, ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        Draw::draw_sprite(&self.ecs, ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() {
    let state = &mut State {
        ecs   : World::new(),
    };

    register_components(&mut state.ecs);

    let c = conf::Conf::new();
    let mut ctx = ContextBuilder::new("first_game", "Tyler_Moroso").conf(c);

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx = ctx.add_resource_path(path);
    }

    let (ref mut ctx, ref mut event_loop) = ctx
        .build()
        .unwrap();

    build_entities(&mut state.ecs, ctx);

    event::run(ctx, event_loop, state).unwrap();
}

fn register_components(ecs: &mut World) {
    ecs.register::<Physics>();
    ecs.register::<Sprite>();
    ecs.register::<Player>();
}

fn build_entities(ecs: &mut World, ctx: &mut Context) {
    MainCharacter::build_entity(ecs, ctx);
}