use specs::{World, WorldExt};

use warmy::{Store, SimpleKey, StoreOpt};

use ggez::{Context, GameResult, timer};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::graphics;

use crate::main_constants::DESIRED_FPS;
use crate::scene::{SceneStack, StartScene};

use Components::register_components;

pub struct State {
    pub ecs:    World,
    pub store: Store<Context, SimpleKey>,
    pub scenes: SceneStack,
}

impl State {
    pub fn new(ctx: &mut Context) -> Self {
        let mut ecs = World::new();
        Components::register_components(&mut ecs);

        let mut store = Store::new(StoreOpt::default()).unwrap();
        let scenes = SceneStack::new(Box::new(StartScene::new(ctx, &mut ecs, &mut store)));

        Self {
            ecs,
            store,
            scenes
        }
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            match self.scenes.update(ctx, &mut self.ecs) {
                Err(e) => {
                    eprintln!("{}", e);
                }
                _ => {
                    ()
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.scenes.draw(ctx, &mut self.ecs);
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key_code: KeyCode,
        key_mods: KeyMods,
        repeat: bool,
    ) {
        self.scenes
            .key_down_event(ctx, key_code, key_mods, repeat, &mut self.ecs);
    }

}