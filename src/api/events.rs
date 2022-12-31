use mlua::{Lua, ToLua};
use rdev::{Button, EventType};
use std::sync::{Arc, Mutex};

use super::Rockstar;

#[derive(Debug, Clone)]
pub enum EventDispatch {
    MousePress(u8),
    MouseRelease(u8),
    MouseMove(f64, f64),

    KeyPress(rdev::Key),
    KeyRelease(rdev::Key),
}

impl<'lua> ToLua<'lua> for EventDispatch {
    fn to_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;

        match self {
            Self::MousePress(button) | Self::MouseRelease(button) => {
                table.set("button", button)?;
            }
            Self::MouseMove(x, y) => {
                table.set("x", x)?;
                table.set("y", y)?;
            }
            Self::KeyPress(key) | Self::KeyRelease(key) => {
                let name = serde_variant::to_variant_name(&key).unwrap();
                table.set("key", name)?;
            }
        }

        Ok(mlua::Value::Table(table))
    }
}

pub fn dispatch_event(event: EventDispatch, lua: &Lua) {
    let str_name = match event {
        EventDispatch::MousePress(_) => "mouse_press",
        EventDispatch::MouseRelease(_) => "mouse_release",
        EventDispatch::MouseMove(_, _) => "mouse_move",
        EventDispatch::KeyPress(_) => "key_press",
        EventDispatch::KeyRelease(_) => "key_release",
    };

    let registry = lua
        .named_registry_value::<_, mlua::Table>("events")
        .unwrap();
    let this_event = registry.get::<_, mlua::Table>(str_name);
    if let Ok(this_event) = this_event {
        for i in 1..=this_event.len().unwrap() {
            let callback = this_event.get::<_, mlua::Function>(i).unwrap();
            callback.call::<_, ()>(event.clone()).unwrap();
        }
    }
}

pub async fn run_event_loop(rockstar: Arc<Mutex<Rockstar>>, lua: Lua) {
    rdev::listen(move |event| {
        //println!("{:?}", event);
        let button_to_u8 = |button: Button| match button {
            Button::Left => 1,
            Button::Right => 2,
            Button::Middle => 3,
            Button::Unknown(n) => n,
        };

        match event.event_type {
            EventType::ButtonPress(button) => {
                let mut rockstar = rockstar.lock().unwrap();
                match button {
                    Button::Left => rockstar.mouse.left_pressed = true,
                    Button::Middle => rockstar.mouse.middle_pressed = true,
                    Button::Right => rockstar.mouse.right_pressed = true,
                    _ => (),
                }

                drop(rockstar);
                let event = EventDispatch::MousePress(button_to_u8(button));
                dispatch_event(event, &lua);
            }
            EventType::ButtonRelease(button) => {
                let mut rockstar = rockstar.lock().unwrap();
                match button {
                    Button::Left => rockstar.mouse.left_pressed = false,
                    Button::Middle => rockstar.mouse.middle_pressed = false,
                    Button::Right => rockstar.mouse.right_pressed = false,
                    _ => (),
                }

                drop(rockstar);
                let event = EventDispatch::MouseRelease(button_to_u8(button));
                dispatch_event(event, &lua);
            }
            EventType::MouseMove { x, y } => {
                let mut rockstar = rockstar.lock().unwrap();
                rockstar.mouse.mouse_x = x;
                rockstar.mouse.mouse_y = y;

                drop(rockstar);
                let event = EventDispatch::MouseMove(x, y);
                dispatch_event(event, &lua);
            }
            EventType::KeyPress(key) => {
                let event = EventDispatch::KeyPress(key);
                dispatch_event(event, &lua);
            }
            EventType::KeyRelease(key) => {
                let event = EventDispatch::KeyRelease(key);
                dispatch_event(event, &lua);
            }
            _ => (),
        }
    })
    .unwrap()
}
