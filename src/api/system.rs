use mlua::UserData;

#[derive(Debug, Clone)]
pub struct System;

impl UserData for System {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_function_get("os", |_, _| Ok(std::env::consts::OS));
    }
}
