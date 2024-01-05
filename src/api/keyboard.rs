use mlua::UserData;
use rdev::EventType;

#[derive(Debug, Clone)]
pub struct Keyboard {
    pub pressed_keys: Vec<rdev::Key>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            pressed_keys: Vec::new(),
        }
    }
}

impl UserData for Keyboard {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("pressed_keys", |_, this| {
            let mut pressed_keys = this
                .pressed_keys
                .iter()
                .map(|key| crate::util::key_to_string(*key))
                .collect::<Vec<_>>();

            pressed_keys.sort();
            pressed_keys.dedup();

            Ok(pressed_keys)
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_function("press", |_, key: String| async move {
            let cheating = crate::util::string_to_key(key)
                .ok_or_else(|| mlua::Error::external("invalid key"))?;

            rdev::simulate(&EventType::KeyPress(cheating))
                .map_err(|_| mlua::Error::external("failed to press key"))?;
            //tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            rdev::simulate(&EventType::KeyRelease(cheating))
                .map_err(|_| mlua::Error::external("failed to release key"))?;

            Ok(())
        });
    }
}
