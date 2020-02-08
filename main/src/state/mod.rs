use specs::{World, WorldExt};
use warmy::{Store, SimpleKey, StoreOpt};
use ggez::{Context, GameResult, graphics, timer};
use ggez::input::keyboard;
use crate::input::get_current_input;
use crate::draw;
use crate::main_constants::DESIRED_FPS;
use crate::scene::{SceneStack, StartScene};

pub struct State {
    pub ecs:    World,
    pub store: Store<Context, SimpleKey>,
    pub scenes: SceneStack,
}

impl State {
    pub fn new(ctx: &mut Context) -> State {
        let mut ecs = World::new();
        let store = Store::new(StoreOpt::default()).unwrap();
        let scenes = SceneStack::new(Box::new(StartScene::new(ctx, &mut ecs)));

        State {
            ecs,
            store,
            scenes
        }
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let current_keys = keyboard::pressed_keys(ctx);
            self.ecs.insert( get_current_input(ctx));
            Systems::run_systems(&mut self.ecs);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        draw::draw_sprite(&self.ecs, ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}