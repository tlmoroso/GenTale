use ggez::Context;
use ggez::input::keyboard;
use ggez::event::KeyCode;

extern crate Components;

pub fn get_current_input(ctx: &mut Context) -> Vec<KeyCode>{
    let mut current_inputs = Vec::new();
    for key in keyboard::pressed_keys(ctx) {
        current_inputs.push(key.clone());
    }
    return current_inputs;
}