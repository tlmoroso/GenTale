use std::fmt::Debug;
use ggez::{Context, GameResult};
use specs::World;
use ggez::event::{KeyCode, KeyMods};
mod start_scene;
pub use self::start_scene::StartScene;

pub trait Scene: Debug {
    fn update(&mut self, ctx: &mut Context, world: &mut World) -> Result<Transition, String>;
    fn draw(&mut self, ctx: &mut Context, world: &mut World) -> GameResult;
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key_code: KeyCode,
        key_mods: KeyMods,
        repeat: bool,
        world: &mut World,
    ) -> Result<Transition, String>;
    fn name(&self) -> &str;
    fn draw_previous(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
    Replace(Box<dyn Scene>),
    MultiReplace(Vec<Box<dyn Scene>>, u32),
}

pub struct SceneStack {
    scenes: Vec<Box<dyn Scene>>,
}

impl SceneStack {
    pub fn new(scene: Box<dyn Scene>) -> Self {
        Self {
            scenes: vec![scene],
        }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, scene: Box<dyn Scene>) {
        self.scenes.push(scene)
    }

    #[allow(dead_code)]
    pub fn pop(&mut self) {
        self.scenes.pop();
    }

    pub fn update(&mut self, ctx: &mut Context, world: &mut World) -> GameResult {
        let scene = self.mut_scene();
        let trans = scene.update(ctx, world).unwrap();
        self.switch(trans);
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, world: &mut World) -> GameResult {
        SceneStack::draw_scenes(&mut self.scenes, ctx, world)
    }

    fn draw_scenes(
        scenes: &mut [Box<dyn Scene>],
        ctx: &mut Context,
        world: &mut World,
    ) -> GameResult {
        assert!(!scenes.is_empty());
        if let Some((current, rest)) = scenes.split_last_mut() {
            if current.draw_previous() {
                SceneStack::draw_scenes(rest, ctx, world)?
            }
            current.draw(ctx, world)?
        }
        Ok(())
    }

    fn mut_scene(&mut self) -> &mut dyn Scene {
        &mut **self.scenes.last_mut().expect("Not scene in stack")
    }

    pub fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key_code: KeyCode,
        key_mods: KeyMods,
        repeat: bool,
        world: &mut World,
    ) {
        let scene = self.mut_scene();
        let trans = scene
            .key_down_event(ctx, key_code, key_mods, repeat, world)
            .unwrap();
        self.switch(trans);
    }

    fn switch(&mut self, trans: Transition) {
        match trans {
            Transition::Push(scene) => {
                self.scenes.push(scene);
            }
            Transition::Pop => {
                if self.scenes.pop().is_none() {
                    eprintln!("Stack doesn't have scene for pop");
                }
            }
            Transition::Replace(scene) => {
                if self.scenes.pop().is_none() {
                    eprintln!("Stack doesn't have scene for replace");
                }
                self.scenes.push(scene)
            }
            Transition::MultiReplace(scenes, mut num) => {
                while num > 0 {
                    if self.scenes.pop().is_none() {
                        eprintln!("Stack doesn't have scene for replace");
                    }
                    num -= 1;
                }

                self.scenes.extend(scenes)
            }
            Transition::None => {}
        };
    }
}
