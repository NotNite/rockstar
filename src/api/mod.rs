use mlua::{prelude::LuaFunction, UserData};

pub mod events;
mod keyboard;
mod mouse;
mod screen;

#[derive(Clone, Debug)]
pub struct Rockstar {
    pub mouse: mouse::Mouse,
    pub screen: screen::Screen,
    pub keyboard: keyboard::Keyboard,
}

impl Rockstar {
    pub fn new() -> Self {
        Self {
            mouse: mouse::Mouse::new(),
            screen: screen::Screen,
            keyboard: keyboard::Keyboard,
        }
    }
}

impl UserData for Rockstar {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("mouse", |_, rockstar| Ok(rockstar.mouse.clone()));
        fields.add_field_method_get("screen", |_, rockstar| Ok(rockstar.screen.clone()));
        fields.add_field_method_get("keyboard", |_, rockstar| Ok(rockstar.keyboard.clone()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("sleep", |_, ms: u64| {
            std::thread::sleep(std::time::Duration::from_millis(ms));
            Ok(())
        });

        methods.add_function("on", |lua, (event, callback): (String, LuaFunction)| {
            let registry = match lua.named_registry_value("events") {
                Ok(registry) => registry,
                Err(_) => {
                    let registry = lua.create_table()?;
                    lua.set_named_registry_value("events", registry.clone())?;
                    registry
                }
            };

            let this_event = match registry.get::<_, mlua::Table>(event.clone()) {
                Ok(this_event) => this_event,
                Err(_) => {
                    let this_event = lua.create_table()?;
                    registry.set(event.clone(), this_event.clone())?;
                    this_event
                }
            };

            this_event.set(this_event.len()? + 1, callback)?;
            registry.set(event, this_event)?;
            lua.set_named_registry_value("events", registry)?;

            Ok(())
        });
    }
}
