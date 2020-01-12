use ggez::*;
use specs::prelude::*;
#[macro_use]
extern crate specs_derive;

extern crate Components;
use Components::Physics::Physics;

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

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        Ok(())
    }
}

pub fn main() {
    let state = &mut State {
        ecs   : World::new(),
    };

    register_components(&mut state.ecs);

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("first_game", "Tyler_Moroso")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}

fn register_components(ecs: &mut World) {
    ecs.register::<Physics>()
}