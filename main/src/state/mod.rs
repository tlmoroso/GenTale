use specs::World;
use warmy::{Store, SimpleKey};
use ggez::{Context, GameResult, graphics};
use ggez::input::keyboard;
use crate::input::get_current_input;
use crate::draw;

pub struct State {
    pub ecs:    World,
    pub store: Store<Context, SimpleKey>
}

impl State {}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let current_keys = keyboard::pressed_keys(ctx);
        self.ecs.insert( get_current_input(ctx));
        Systems::run_systems(&mut self.ecs);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        draw::draw_sprite(&self.ecs, ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}