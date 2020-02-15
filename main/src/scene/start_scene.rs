use std::fmt;

use Entities::button::Button;
use Entities::MyEntity;

use Systems::draw;

use Components::clickable::Clickable;

use specs::{Dispatcher, World, WorldExt, DispatcherBuilder, Entity};
use crate::scene::{Scene, Transition};
use ggez::{Context, GameError};
use ggez::graphics;
use ggez::event::{KeyMods, KeyCode};
use serde::Deserialize;

use warmy::{SimpleKey, Store, Res};
use warmy::json::Json;

const JSON_PATH: &str = "\\main\\resources\\JSON\\start_scene.json";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
struct StartSceneJSON {
    buttons: Vec<String>,
}

pub struct StartScene<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    clickables: Vec<Entity>,
    current_clickable: usize
}

impl <'a, 'b> StartScene <'a, 'b> {
    pub fn new(ctx: &mut Context, ecs: &mut World, store: &mut Store<Context, SimpleKey>) -> Self {
        let mut button_paths: Vec<String> = vec!{};
        let scene_json: Result<Res<StartSceneJSON>, _> = store.get_by(&SimpleKey::from_path(JSON_PATH), ctx, Json);
        match scene_json {
            Ok(scene_default) => {
                button_paths = scene_default.borrow().buttons.clone();
            }
            Err(e) => eprintln!("{}", e)
        };

        let mut buttons: Vec<Entity> = Vec::new();
        for path in button_paths {
            let button = Button::build_entity(ecs, store, ctx, path.as_str());
            match button {
                Some(b) => {
                    buttons.push(b);
                }
                None => {
                    panic!("Could not build button entity in Start Screen");
                }
            }

        }

        let mut dispatcher = DispatcherBuilder::new()
            .build();

        dispatcher.setup(ecs);
        Self {
            dispatcher,
            clickables: buttons,
            current_clickable: 0
        }
    }
}

impl<'a, 'b> Scene for StartScene<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context, ecs: &mut World) -> Result<Transition, String> {
        self.dispatcher.dispatch(ecs);

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, ecs: &mut World) -> Result<(), GameError> {
        draw::draw_sprite(ecs, ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key_code: KeyCode, _key_mods: KeyMods, _repeat: bool, world: &mut World) -> Result<Transition, String> {
        let mut clickable_storage = world.write_storage::<Clickable>();

        match key_code {
            KeyCode::Down => {
                let mut prev_entity = clickable_storage.get_mut(self.clickables[self.current_clickable]).unwrap();
                prev_entity.is_focused = false;
                self.current_clickable = (self.current_clickable + 1) % self.clickables.len();
                let mut next_entity = clickable_storage.get_mut(self.clickables[self.current_clickable]).unwrap();
                next_entity.is_focused = true;
                Ok(Transition::None)
            }
            KeyCode::Up => {
                let mut prev_entity = clickable_storage.get_mut(self.clickables[self.current_clickable]).unwrap();
                prev_entity.is_focused = false;
                

                if self.current_clickable > 0 {
                    self.current_clickable = (self.current_clickable - 1) % self.clickables.len();
                } else {
                    self.current_clickable = self.clickables.len() - 1;
                }

                let mut next_entity = clickable_storage.get_mut(self.clickables[self.current_clickable]).unwrap();
                next_entity.is_focused = true;
                Ok(Transition::None)
            }
            KeyCode::Return => {
                let selected_entity = clickable_storage.get_mut(self.clickables[self.current_clickable]).unwrap();
                Ok(Transition::Pop)
            }
            _ => Ok(Transition::None)
        }
    }

    fn name(&self) -> &str {
        "Start Screen"
    }
}

impl<'a, 'b> fmt::Debug for StartScene<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
